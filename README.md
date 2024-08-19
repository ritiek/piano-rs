# piano-rs

![Rust Toolchain](https://img.shields.io/badge/rust-stable-brightgreen.svg)
[![Build Status](https://travis-ci.org/ritiek/piano-rs.svg?branch=master)](https://travis-ci.org/ritiek/piano-rs)

A multiplayer piano using UDP sockets that can be played using computer keyboard, in the terminal.

## Screenshots

[Video clip](https://peertube.social/videos/watch/cb98f9b5-5c5b-417b-bde4-94f17533910c)

<img src="https://i.imgur.com/DOx0wWf.png" width="900">

## Compiling

You'll need to have Rust compiler and its package manager, Cargo installed to compile piano-rs.
If you don't have them already, head over to https://rustup.rs/ to run the installer.

You can then compile piano-rs with:

```
$ git clone https://github.com/ritiek/piano-rs
$ cd piano-rs
$ cargo build --release
```

If you're using Ubuntu, you might face the following:

```
error: failed to run custom build command for `alsa-sys v0.1.1`
```

In this case, compiling again after installing `libasound2-dev` should solve the problem:
```
$ sudo apt-get install libasound2-dev
```

## Usage

Once it compiles, run the binary with:
```
$ cargo run --release
```

You can also call the binary directly located in `./target/release/piano-rs`.

Additional options to the compiled binary can be passed with cargo such as:

```
$ cargo run --release -- --help

Play piano in the terminal using PC (computer) keyboard.

USAGE:
    piano-rs [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --host-address <ADDRESS>        Set the host's IP Address and Port to connect to (Default: receiver address)
    -m, --mark-duration <DURATION>      Duration to show piano mark for, in ms (Default: 500)
    -n, --note-duration <DURATION>      Duration to play each note for, where 0 means till the end of note (Default: 0)
    -p, --play-file <FILEPATH>          Play notes from .yml file (Default: None)
    -t, --playback-tempo <AMOUNT>       Set playback speed when playing from file (Default: 1.0)
        --receiver-address <ADDRESS>    Set the IP Address and Port to which the receiver socket will bind to (Default:
                                        0.0.0.0:9999)
    -r, --record-file <FILEPATH>        Record notes to .yml file (Default: None)
        --sender-address <ADDRESS>      Set the IP Address and Port to which the sender socket will bind to. A port of 0
                                        implies to bind on a random unused port (Default: 0.0.0.0:0)
    -s, --sequence <AMOUNT>             Frequency sequence from 0 to 5 to begin with (Default: 2)
    -v, --volume <AMOUNT>               Set initial volume for notes (Default: 1.0)
```

- You can press the keys on your computer keyboard to play the piano notes.

- Increase or decrease the note frequency with <kbd>←</kbd> and <kbd>→</kbd> respectively
  (or hold <kbd>ctrl</kbd> or <kbd>shift</kbd> while playing).

- Adjust the duration for how long the notes play for with <kbd>↑</kbd> and <kbd>↓</kbd>.

- Adjust the volume of the notes with <kbd>-</kbd> and <kbd>+</kbd>.

- You can also record your piano session by passing the command-line argument `-r <path/to/save/notes.yml>`
  and play them later on with `-p <path/to/save/notes.yml>`.

Press the <kbd>Esc</kbd> key to exit the game.

**NOTE:** If you get no sound when you press keys, [try running it in a directory containing the assets directory](https://github.com/ritiek/piano-rs/issues/6#issuecomment-354971861). The note sound files are loaded at runtime and piano-rs will fail to load them if it cannot find the assets
directory in your current working directory.

## Multiplayer

piano-rs is multiplayer! It can also be enjoyed with friends by sharing the same piano session. Here's how to setup:

On the 1st machine, you would launch piano-rs as usual with:
```
$ cargo run --release
```
or
```
$ ./target/release/piano-rs
```

On the 2nd machine, you would then pass the IP address of the receiver socket of the 1st machine, which
by default binds to `0.0.0.0:9999` and can be overriden with `--receiver-address`. That means, you would
run something like this on the 2nd machine to connect to the 1st machine's piano-rs session:
```
$ cargo run --release -- --host-address=192.168.1.3:9999
```
or
```
$ ./target/release/piano-rs --host-address=192.168.1.3:9999
```

Here, 192.168.1.3 is the IP address of the 1st machine.

The 2nd machine should now be connected and will share the same piano-rs session as the host machine.
Any keys you hit, should be marked with a different color indicator.

Similar to the way you connected the 2nd machine, you can connect any number of machines to share
the same piano-rs session!

--------------------

**NOTE:** These multiplayer features do not make use of tokio-rs runtime and instead use `std::net::UdpSocket`
for communication, which comes included with the Rust standard library. The major limitation of relying on
`std::net::UdpSocket` is that the network requests are handled sequentially on the basis of first come,
first serve. This would be a problem if hundreds of players are connected to the same piano-rs session and
are hitting the keys at the same time. Obviously, we could acheive much better performance if we were to
handle network requests asynchronously with [tokio-rs](https://github.com/tokio-rs/tokio) and
[futures](https://docs.rs/futures/0.1.29/futures/). Unfortunately, these awesome libraries
have a bit of learning curve which I don't have the time to go through at the moment! It will be awesome if
someone would like to help here make a transition to asynchronously handle network requests.

The cool devs at tokio-rs have also been trying to lower the learning curve by introducing `async` and `await`
keywords, similar to [Python](https://docs.python.org/3/library/asyncio.html). However, these keywords at the
moment are only available under the recent alpha release of tokio-rs for Rust nightly. See the relevant
[blog post](https://tokio.rs/blog/2019-08-alphas/).

## Running tests

```
$ cargo test
```

## Resources

- piano-rs uses the same note sounds and key bindings as [multiplayerpiano](http://multiplayerpiano.com).
  In fact, the note sound files you see in the [assets](https://github.com/ritiek/piano-rs/tree/master/assets)
  sub-directory are downloaded from multiplayerpiano itself.
  If you're a moderator on their website and got a problem with this, let me know and I'll remove and
  stop using the sound files in this repository.

- You can use this [paste](https://pastebin.com/CX1ew0uB) to learn to play some popular songs. If you're
  interested, I've *transcribed* a few synthesia YouTube videos [in this gist](https://gist.github.com/ritiek/28be91b64ef82f0ff8599c1037e1e05e),
  so they can be played with piano-rs.

## License

`The MIT License`
