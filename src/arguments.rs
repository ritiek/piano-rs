use clap::{Arg, App, ArgMatches};
use clap::value_t;
use std::net::SocketAddr;

pub struct Options {
    pub host_address: SocketAddr,
    pub volume: f32,
    pub record_file: Option<String>,
    pub play_file: Option<String>,
    pub play_file_tempo: f32,
    pub sequence: i8,
    pub note_duration: u64,
    pub mark_duration: u64,
    pub receiver_address: SocketAddr,
    pub sender_address: SocketAddr,
}

impl Options {
    pub fn read() -> Options {
        let arguments = Self::get_arguments();
        let receiver_address = value_t!(arguments.value_of("receiver_address"), SocketAddr)
            .unwrap_or_else(|_| "0.0.0.0:9999".parse().unwrap());

        let parsed_arguments = Options {
            host_address     : value_t!(arguments.value_of("host_address"), SocketAddr)
                                .unwrap_or(receiver_address),
            volume           : value_t!(arguments.value_of("volume"), f32)
                                .unwrap_or(1.0),
            record_file      : value_t!(arguments.value_of("record_file"), String)
                                .ok(),
            play_file        : value_t!(arguments.value_of("play_file"), String)
                                .ok(),
            play_file_tempo  : value_t!(arguments.value_of("play_file_tempo"), f32)
                                .unwrap_or(1.0),
            sequence         : value_t!(arguments.value_of("sequence"), i8)
                                .unwrap_or(2),
            note_duration    : value_t!(arguments.value_of("note_duration"), u64)
                                .unwrap_or(0),
            mark_duration    : value_t!(arguments.value_of("mark_duration"), u64)
                                .unwrap_or(500),
            receiver_address,
            sender_address   : value_t!(arguments.value_of("sender_address"), SocketAddr)
                                .unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap()),
        };

        parsed_arguments
    }

    fn get_arguments<'a>() -> ArgMatches<'a> {
        App::new("piano-rs")
            .version("0.2.0")
            .author("Ritiek Malhotra <ritiekmalhotra123@gmail.com>")
            .about("Play piano in the terminal using PC (computer) keyboard.")

            .arg(Arg::with_name("host_address")
                .long("host-address")
                .value_name("ADDRESS")
                .takes_value(true)
                .help("Set the host's IP Address and Port to connect to (Default: receiver address)"))

            .arg(Arg::with_name("volume")
                .short("v")
                .long("volume")
                .value_name("AMOUNT")
                .takes_value(true)
                .help("Set initial volume for notes (Default: 1.0)"))

            .arg(Arg::with_name("record_file")
                .short("r")
                .long("record-file")
                .value_name("FILEPATH")
                .takes_value(true)
                .help("Record notes to .yml file (Default: None)"))

            .arg(Arg::with_name("play_file")
                .short("p")
                .long("play-file")
                .value_name("FILEPATH")
                .takes_value(true)
                .help("Play notes from .yml file (Default: None)"))

            .arg(Arg::with_name("play_file_tempo")
                .short("t")
                .long("playback-tempo")
                .value_name("AMOUNT")
                .takes_value(true)
                .help("Set playback speed when playing from file (Default: 1.0)"))

            .arg(Arg::with_name("sequence")
                .short("s")
                .long("sequence")
                .value_name("AMOUNT")
                .takes_value(true)
                .help("Frequency sequence from 0 to 5 to begin with (Default: 2)"))

            .arg(Arg::with_name("note_duration")
                .short("n")
                .long("note-duration")
                .value_name("DURATION")
                .takes_value(true)
                .help("Duration to play each note for, where 0 means till the end of note (Default: 0)"))

            .arg(Arg::with_name("mark_duration")
                .short("m")
                .long("mark-duration")
                .value_name("DURATION")
                .takes_value(true)
                .help("Duration to show piano mark for, in ms (Default: 500)"))

            .arg(Arg::with_name("receiver_address")
                .long("receiver-address")
                .value_name("ADDRESS")
                .takes_value(true)
                .help("Set the IP Address and Port to which the receiver socket will bind to (Default: 0.0.0.0:9999)"))

            .arg(Arg::with_name("sender_address")
                .long("sender-address")
                .value_name("ADDRESS")
                .takes_value(true)
                .help("Set the IP Address and Port to which the sender socket will bind to. A port of 0 implies to bind on a random unused port (Default: 0.0.0.0:0)"))

            .get_matches()
    }

}
