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
  cargo d -p pomo-cli -r --no-deps --open

# Default `just` command to list all commands
default:
  just --list

f:
  cargo fmt

# Run bin in pomo-cli/src/bin/pomo_cli.rs
r:
  cargo r -p pomo-cli

# Run with `RUST_BACKTRACE=1` environment variable to display a backtrace
rtrace:
  RUSTBACKTRACE=1 cargo r -p pomo-cli

# Test lib pomo_cli
t:
  cargo test -p pomo-cli

# Watch and Run bin in pomo-cli/src/bin/pomo_cli.rs
w:
  cargo watch -x 'r -p pomo-cli'

# Watch and test lib
wt:
  cargo watch -x 't -p pomo-cli'

# Watch and pass cli arguments
wcli:
  cargo watch -x 'run -- --task Study --intervals 4'
