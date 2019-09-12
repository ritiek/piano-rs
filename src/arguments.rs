use clap::{Arg, App, ArgMatches};


pub fn get_arguments<'a>() -> ArgMatches<'a> {
    App::new("piano-rs")
        .version("0.1.0")
        .author("Ritiek Malhotra <ritiekmalhotra123@gmail.com>")
        .about("Play piano in the terminal using PC (computer) keyboard.")

        .arg(Arg::with_name("volume")
            .short("v")
            .long("volume")
            .value_name("AMOUNT")
            .takes_value(true)
            .help("Set intial volume for notes (Default: 1.0)"))

        .arg(Arg::with_name("record")
            .short("r")
            .long("record-file")
            .value_name("FILEPATH")
            .takes_value(true)
            .help("Record notes to .yml file (Default: none)"))

        .arg(Arg::with_name("play")
            .short("p")
            .long("play-file")
            .value_name("FILEPATH")
            .takes_value(true)
            .help("Play notes from .yml file (Default: none)"))

        .arg(Arg::with_name("color")
            .short("c")
            .long("color")
            .value_name("COLOR")
            .takes_value(true)
            .help("Color of block to generate when a note is played (Default: \"red\")"))

        .arg(Arg::with_name("tempo")
            .short("t")
            .long("playback-tempo")
            .value_name("AMOUNT")
            .takes_value(true)
            .help("Set playback speed when playing from file (Default: 1.0)"))

        .arg(Arg::with_name("replaycolor")
            .short("x")
            .long("replay-color")
            .value_name("COLOR")
            .takes_value(true)
            .help("Color of block to generate when notes are played from file (Default: \"blue\")"))

        .arg(Arg::with_name("sequence")
            .short("s")
            .long("sequence")
            .value_name("SEQUENCE")
            .takes_value(true)
            .help("Frequency sequence from 0 to 5 to begin with (Default: 2)"))

        .arg(Arg::with_name("noteduration")
            .short("n")
            .long("note-duration")
            .value_name("DURATION")
            .takes_value(true)
            .help("Duration to play each note for, where 0 means till the end of note (Default: 0)"))

        .arg(Arg::with_name("markduration")
            .short("m")
            .long("mark-duration")
            .value_name("DURATION")
            .takes_value(true)
            .help("Duration to show piano mark for, in ms (Default: 500)"))

        .get_matches()
}
