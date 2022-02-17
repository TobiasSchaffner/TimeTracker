# Time Tracker

A simple screen time tracker.

It tracks the time that your screen was unlocked.

There is a daemon (time-tracker-daemon) running in the background that writes
the screen times to ~/.config/



## Build Project

cargo build --release

## Build Debian Package

cargo install deb
cargo deb -p package

## Development

Use run.sh to build and install the .deb and to run time-tracker
