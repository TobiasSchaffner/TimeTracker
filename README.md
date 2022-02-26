# Time Tracker

A simple screen time tracker.

It tracks the time that your screen was unlocked.

There is a daemon (time-tracker-daemon) running in the background that writes
the active screen times to ~/.config/

Use the time-tracker cli tool show the total active times per day.
Per default the tool will show the last seven days.

## Usage

```
USAGE:
    time-tracker [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -n, --number <NUMBER>    The number of dates that should be printed [default: 7]
    -s, --start <START>      The start of the printed range [default: -7]
    -V, --version            Print version information
```

## Build Project
```
cargo build --release
```
## Build Debian Package
```
cargo install deb
cargo deb -p package
```
## Development

Use run.sh to build and install the .deb and to run time-tracker

## Troubleshooting

Check if the daemon is running and if it throws any error messages:
```
systemctl --user status time-tracker-daemon
```

Check if the daemon is writing to the config files:
```
ls -la ~/.config/time-tracker/
```
