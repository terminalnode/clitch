![cargo build](https://github.com/terminalnode/clitch/actions/workflows/cargo-build.yml/badge.svg)
![cargo fmt](https://github.com/terminalnode/clitch/actions/workflows/cargo-fmt.yml/badge.svg)

# Clitch
A CLI watch, exposing a number (two, to be precise) of subcommands for working with time.

## Build and install
Just build the program with `cargo build -r`, then move the resulting binary from
`./target/release/clitch` to some desired directory on your `$PATH`.

## Subcommands
Below follows a list of available subcommands with examples. For a more in-depth explanation, each subcommand
has a help menu that can be accessed either with `clitch help <subcommand>` or `clitch <subcommand> -h`.

If no subcommand is specified, the default subcommand is `clock`. But to show the `clock` help you still need to
specify its name explicitly (e.g. `clitch help clock` or `clitch clock -h`).

### `clock`
Currently only shows local time in the format `HH:MM:SS`.
The idea is to eventually expose functions similar to `countdown`, mainly such as `-c` / `--continuous`.

```shell
$ clitch clock
15:36:16
```

### `countdown`
This subcommand is identical to [countdown.rs](https://github.com/terminalnode/countdown.rs).

```shell
$ clitch countdown -d 2024-03-23 -t 10:00 -v
Now:    2024-03-18 10:40:44 (+01:00)
Target: 2024-03-23 10:00:00 (CET)
4 days 23:19:15

$ clitch countdown -d 2024-03-23 -t 10:00 -cv
Now:    2024-03-18 10:40:47 (+01:00)
Target: 2024-03-23 10:00:00 (CET)
4 days 23:19:12
4 days 23:19:11
4 days 23:19:10
^C

# This variant will update the remaining time on the same line, instead of printing a new line every second
$ clitch countdown -d 2024-03-23 -t 10:00 -cvo
Now:    2024-03-18 10:40:58 (+01:00)
Target: 2024-03-23 10:00:00 (CET)
4 days 23:18:54
```