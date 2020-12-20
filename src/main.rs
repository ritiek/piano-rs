use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::net::SocketAddr;
use std::io::{stdout, Write, Result};
use std::path::PathBuf;
use crossterm::{
    cursor,
    input,
    execute,
    RawScreen,
    Clear,
    ClearType,
    InputEvent,
    SyncReader,
};
use crossterm_style::Color;

use piano_rs::arguments::Options;
use piano_rs::game::{
    PianoKeyboard,
    GameEvent,
    Note,
    NoteReader,
};
use piano_rs::network::{
    NetworkEvent,
    Receiver,
    Sender,
};

fn handle_network_receive_event(
    keyboard: &Arc<Mutex<PianoKeyboard>>,
    event_sender: &Arc<Mutex<Sender>>,
    event_receiver: &Receiver,
) {
    let data = event_receiver.poll_event().unwrap();
    match data.event {
        NetworkEvent::PlayerJoin(port) => {
            let remote_receiver_addr: SocketAddr = format!("{}:{}", data.src.ip(), port)
                .parse()
                .unwrap();

            event_sender.lock().unwrap()
                .register_remote_socket(
                    event_receiver.socket.local_addr().unwrap().port(), remote_receiver_addr
                )
                .unwrap();
        }
        NetworkEvent::Peers(port, mut peers) => {
            peers[0] = format!("{}:{}", data.src.ip(), port).parse().unwrap();
            event_sender.lock().unwrap().peer_addrs = peers;
        }
        NetworkEvent::ID(id) => {
            keyboard.lock().unwrap().set_note_color(match id {
                0 => Color::Blue,
                1 => Color::Red,
                2 => Color::Green,
                3 => Color::Yellow,
                4 => Color::Cyan,
                5 => Color::Magenta,
                _ => Color::Black,
            });
        }
        NetworkEvent::Note(note) => {
            keyboard.lock().unwrap().play_note(note);
        }
       _ => { },
    }
}

fn game_loop(stdin: &mut SyncReader, keyboard: &Arc<Mutex<PianoKeyboard>>, event_sender: &Arc<Mutex<Sender>>) {
    /* let duration = Duration::from_nanos(1000); */

    loop {
        if let Some(event) = stdin.next() {
            if let InputEvent::Keyboard(key) = event {
                match keyboard.lock().unwrap().process_key(key) {
                    Some(GameEvent::Note(note)) => {
                        event_sender.lock().unwrap().tick(note).unwrap();
                    }
                    Some(GameEvent::Quit) => break,
                    None => { },
                }
            }
        }
    }
}

fn play_from_file(play_file: PathBuf, tempo: f32, keyboard: &Arc<Mutex<PianoKeyboard>>, event_sender: &Arc<Mutex<Sender>>) {
    let file_base_notes = NoteReader::from(play_file);
    for file_base_note in file_base_notes.parse_notes() {
        let note = Note::from(
            file_base_note.base_note.as_str(),
            keyboard.lock().unwrap().color,
            file_base_note.duration,
        ).unwrap();
        let normalized_delay = Duration::from_millis(
            (file_base_note.delay.as_millis() as f32 / tempo) as u64
        );
        thread::sleep(normalized_delay);
        event_sender.lock().unwrap().tick(note).unwrap();
    }
}

fn main() -> Result<()> {
    let arguments = Options::read();

    let receiver_address = arguments.receiver_address;
    let event_receiver = Receiver::new(receiver_address)?;
    let event_sender = Arc::new(Mutex::new(Sender::new(arguments.sender_address, arguments.host_address)?));
    let event_sender_clone = event_sender.clone();

    execute!(stdout(), Clear(ClearType::All)).unwrap();

    let _raw = RawScreen::into_raw_mode();

    let keyboard = Arc::new(Mutex::new(PianoKeyboard::new(
        arguments.sequence,
        arguments.volume,
        Duration::from_millis(arguments.note_duration),
        Duration::from_millis(arguments.mark_duration),
        Color::Blue,
    )));

    keyboard.lock().unwrap().draw().unwrap();

    let cloneboard = keyboard.clone();

    thread::spawn(move || {
        loop {
            handle_network_receive_event(
                &cloneboard,
                &event_sender_clone,
                &event_receiver
            );
        }
    });

    event_sender.lock().unwrap().register_self(arguments.receiver_address.port())?;

    if let Some(v) = arguments.record_file {
        keyboard.lock().unwrap().set_record_file(PathBuf::from(v));
    }

    if let Some(v) = arguments.play_file {
        let play_file = PathBuf::from(v);
        let tempo = arguments.play_file_tempo;
        let fileboard = keyboard.clone();
        let file_notes_sender = event_sender.clone();
        thread::spawn(move || play_from_file(
            play_file,
            tempo,
            &fileboard,
            &file_notes_sender
        ));
    }

    let input = input();
    let mut sync_stdin = input.read_sync();

    let cursor = cursor();
    cursor.hide().unwrap_or_default();

    game_loop(&mut sync_stdin, &keyboard, &event_sender);

    Ok(())
}
