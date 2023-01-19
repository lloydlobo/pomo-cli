# pompom

This crate is a simple pomodoro cli based terminal timer.

# Dependencies

-   Works on Linux and MacOS. Currently unimplemented for Windows except for WSL.
    ODO: Install `libdbus-1` to integrate pompom with linux's notification system.

# Installation

Works both on stable and nightly rust.

Install the application with the command:

_Stable_

```terminal
$ cargo install pompom
```

_Nightly_

```terminal
$ cargo +nightly install pompom
```

# Using pompom

Run pompom directly in your terminal. It currently defaults to:

-   25 minutes of work time.
    _ 5 minutes of break.
    _ 20 minutes of long break.

```terminal
$ pompom
```

# Flags

To customize the pompom settings, you can pass flags into the terminal.
_ `-w` | `--work` - sets the work time.
_ `-s` | `--shortbreak` - sets the short break time. \* `-l` | `--longbreak` - sets the long break time.

# Examples

```terminal
$ pompom -w 45 -s 15 -l 25
```

```terminal
$ pompom --work 45 --shortbreak 15 --longbreak 25
```
