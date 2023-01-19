# Cargo build binary
b:
  cargo b

# Cargo build binary in release mode
br:
  cargo b -r

# Cargo check
c:
  cargo check

# Lint with clippy
cl:
  cargo clippy

# Lint and fix with clippy
clf:
  cargo clippy fix

# Create and serve rustdoc for pomodoro w/o dependecies
d:
  cargo d -p pompom -r --no-deps --open

# Default `just` command to list all commands
default:
  just --list

f:
  cargo fmt

# Run bin from pompom/src/bin/pompom.rs
r:
  cargo r -p pompom

# Run bin from pompom/src/bin/pompom.rs in release mode
rr:
  cargo r -r -p pompom

# Run with `RUST_BACKTRACE=1` environment variable to display a backtrace
rtrace:
  RUSTBACKTRACE=1 cargo r -p pompom

# Test lib pompom
t:
  cargo test -p pompom

# Watch and Run bin in pompom/src/bin/pompom.rs
w:
  cargo watch -x 'r -p pompom'

# Watch and run to see help cli usage stdout
whelp:
  cargo watch -x 'r -- --help'

# Watch and test lib
wt:
  cargo watch -x 't -p pompom'

# Watch and pass cli arguments
wcli:
  cargo watch -x 'run -- --task Study --intervals 4'
