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

<details>
<summary><b>clock</b></summary>
<p>
Shows local time in the format HH:MM:SS.
</p>

```shell
$ clitch clock
20:26:23

$ clitch clock -c
20:26:26
20:26:27
20:26:28
20:26:29
^C

# This variant will update the remaining time on the same line, instead of printing a new line every second
$ clitch clock -cof '(%a) %d %h %H:%M:%S'
(Thu) 21 Mar 20:26:35
```
</details>

<details>
<summary><b>countdown</b></summary>
<p>
This subcommand is identical to <a href="https://github.com/terminalnode/countdown.rs">countdown.rs</a>, as the
name implies it counts the time between now and a provided target time.
</p>

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
</details>
