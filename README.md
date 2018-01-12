# piano-rs

[![Build Status](https://travis-ci.org/ritiek/piano-rs.svg?branch=master)](https://travis-ci.org/ritiek/piano-rs)

Play piano in the terminal using PC (computer) keyboard.

## Screenshots

<img src="http://i.imgur.com/33s2XDW.png" width="900">

## Compiling

```
$ git clone https://github.com/ritiek/piano-rs
$ cd piano-rs
$ cargo build --release
```
## Usage

Once it compiles, run the binary in `./target/release/piano-rs`:

```
$ ./target/release/piano-rs -h

Play piano in the terminal using PC keyboard.

USAGE:
    piano-rs [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --color <COLOR>               Color of block to generate when a note is played (Default: "red")
    -m, --mark-duration <DURATION>    Duration to show piano mark for, in ms (Default: 500)
    -n, --note-duration <DURATION>    Duration to play each note for, where 0 means till the end of note (Default: 0)
    -s, --sequence <SEQUENCE>         Frequency sequence from 0 to 5 to begin with (Default: 2)
```

- Press keys on your PC keyboard to play the notes.
- Adjust note frequency using <kbd>←</kbd> and <kbd>→</kbd> (or hold <kbd>ctrl</kbd> or <kbd>shift</kbd> respectively while playing).
- Adjust note duration using <kbd>↑</kbd> and <kbd>↓</kbd>.

## Running tests

```
$ cargo test
```

## Resources

- piano-rs uses the same notes and key bindings as [multiplayerpiano.com](http://multiplayerpiano.com).

- You can use this [paste](https://pastebin.com/CX1ew0uB) to learn to play various popular songs.

## License

`The MIT License`
