Rust Keylogger
==============

This is a keylogger for Linux written in Rust, ported from my [original keylogger](https://github.com/gsingh93/simple-key-logger) in C. It works by reading directly from the keyboard device in `/dev/input/`. The keylogger attempts to detect the keyboard device upon startup, but if one cannot be detected or if multiple are detected, you must specify the path to the device file manually.

Only the US keyboard layout is supported. See [input.rs](https://github.com/gsingh93/keylogger/blob/master/src/input.rs) if you are interested in adding mappings for other keyboard layouts.

I am not responsible for how you use this tool.

## Installation

Clone the repository:

```
$ git clone git@github.com:gsingh93/keylogger.git
$ cd keylogger
```

Build the code:

```$ cargo build --release```

You can run the code with Cargo or directly from the target directory. Note that the keylogger must be run as the root user:

```
$ sudo cargo run --release -- -h
$ sudo ./target/release/keylogger -h
```

You can move the `keylogger` binary wherever you want. For example, you can put it in `/usr/local/bin` or in any other directory in your path.

## Usage

```
$ sudo cargo run -- -h

Usage: target/release/keylogger [options]

Options:
    -h --help           prints this help message
    -v --version        prints the version
    -d --device DEVICE  specify the device file
    -f --file FILE      specify the file to log to
```

If the `-f` flag is not specified, the file `keys.log` is used.

If you would like to run the keylogger in the background, append an `&` to the end of the command. If you would like to run the keylogger as a daemon or at startup, use init script/service manager that comes with your distro. An example `systemd` file is provided.

## License

[MIT](https://github.com/gsingh93/keylogger/blob/master/LICENSE.txt)
