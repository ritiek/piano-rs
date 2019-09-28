use rustbox::{Color, RustBox};
use std::default::Default;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::net::SocketAddr;
use std::io::Result;
use std::path::PathBuf;

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

fn main() -> Result<()> {
    let arguments = Options::read();
    // A workaround to stop cracking noise after note ends (issue #4)
    let blank_point = rodio::default_output_device().unwrap();
    let blank_sink = rodio::Sink::new(&blank_point);
    let blank_source = rodio::source::SineWave::new(0);
    blank_sink.append(blank_source);

    let event_receiver = Receiver::new(arguments.receiver_address)?;
    let event_sender = Arc::new(Mutex::new(Sender::new(arguments.sender_address, arguments.host_address)?));
    let event_sender_clone = event_sender.clone();

    let rustbox = Arc::new(Mutex::new(
        RustBox::init(Default::default()).unwrap()
    ));

    let keyboard = Arc::new(Mutex::new(PianoKeyboard::new(
        arguments.sequence,
        arguments.volume,
        Duration::from_millis(arguments.note_duration),
        Duration::from_millis(arguments.mark_duration),
        Color::Blue,
    )));

    keyboard.lock().unwrap().draw(&rustbox);

    let clonebox = rustbox.clone();
    let cloneboard = keyboard.clone();

    thread::spawn(move || {
        loop {
            let data = event_receiver.poll_event().unwrap();
            match data.event {
                NetworkEvent::PlayerJoin => {
                    let remote_receiver_addr: SocketAddr = format!("{}:9999", data.src.ip())
                        .parse()
                        .unwrap();

                    event_sender_clone.lock().unwrap()
                        .register_remote_socket(remote_receiver_addr)
                        .unwrap();
                }
                NetworkEvent::Peers(mut peers) => {
                    peers[0] = format!("{}:9999", data.src.ip()).parse().unwrap();
                    event_sender_clone.lock().unwrap().peer_addrs = peers;
                }
                NetworkEvent::ID(id) => {
                    cloneboard.lock().unwrap().set_note_color(match id {
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
                    cloneboard.lock().unwrap().play_note(note, &clonebox);
                }
               _ => { },
            }
        }
    });

    event_sender.lock().unwrap().register_self()?;

    if let Some(v) = arguments.record_file {
        keyboard.lock().unwrap().set_record_file(PathBuf::from(v));
    }

    if let Some(playfile) = arguments.play_file {
        let tempo = arguments.play_file_tempo;
        let file_base_notes = NoteReader::from(PathBuf::from(playfile));
        let color = keyboard.lock().unwrap().color;
        let file_notes_sender = event_sender.clone();

        thread::spawn(move || {
            for file_base_note in file_base_notes.parse_notes() {
                let note = Note::from(
                    file_base_note.base_note.as_str(),
                    color,
                    file_base_note.duration,
                ).unwrap();
                let normalized_delay = Duration::from_millis(
                    (file_base_note.delay.as_millis() as f32 / tempo) as u64
                );
                thread::sleep(normalized_delay);
                file_notes_sender.lock().unwrap().tick(note).unwrap();
            }
        });
    }


    let duration = Duration::from_nanos(1000);
    loop {
        let event = rustbox.lock().unwrap().peek_event(duration, false);
        match event {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match keyboard.lock().unwrap().process_key(key) {
                    Some(GameEvent::Note(note)) => {
                        event_sender.lock().unwrap().tick(note).unwrap();
                    }
                    Some(GameEvent::Quit) => break,
                    None => { },
                };
            }
            Err(e) => panic!("{}", e),
            _ => { },
        }
    }
    Ok(())
}

