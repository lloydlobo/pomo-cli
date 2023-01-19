# pompom

This crate is a simple pomodoro cli based terminal timer.

## Dependencies

-   Works on Linux and MacOS, and Windows Subsystem For Linux (WSL).
-   TODO: Install `libdbus-1` to integrate pompom with linux's notification system.

## Installation

Works both on stable and nightly rust.

Install the application with the command:

### Crates.io

_Stable_

```terminal
$ cargo install pompom
```

_Nightly_

```terminal
$ cargo +nightly install pompom
```

### Install or reinstall the package in the source current directory

You'll need to either Clone or Download the source code before building & installing the binary.

_1.Build binary._

```terminal
$ cd pompom
$ cargo build --release --bin pompom
```

_2. Install binary to your OS._

```terminal
$ cargo install --path .
```

## Using pompom

Run pompom directly in your terminal. It currently defaults to:

-   25 minutes of work time.
-   5 minutes of break.
-   20 minutes of long break.

```terminal
$ pompom
```

### Flags

To customize the pompom settings, you can pass flags into the terminal.

-   `-w` | `--work` - sets the work time.
-   `-s` | `--shortbreak` - sets the short break time.
-   `-l` | `--longbreak` - sets the long break time.

## Examples

```terminal
$ pompom -w 45 -s 15 -l 25
```

```terminal
$ pompom --work 45 --shortbreak 15 --longbreak 25
```
