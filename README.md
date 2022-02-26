# Time Tracker

A plain simple screen time tracker for linux.

It tracks the time that your screen was unlocked.

The tool was created as the other solutions available e.g. activity watch are
too sophisticated for my usecase.

## Usage

Use the time-tracker cli tool show the total active times per day.

Per default the tool will show the last seven days.

```
USAGE:
    time-tracker [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -n, --number <NUMBER>    The number of dates that should be printed [default: 7]
    -s, --start <START>      The start of the printed range [default: -7]
    -v, --verbose            Print all breaks
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

There is a daemon (time-tracker-daemon) running in the background that writes
the active screen times to ~/.config/

Check if the daemon is running and if it throws any error messages:
```
systemctl --user status time-tracker-daemon
```

Check if the daemon is writing to the config files:
```
ls -la ~/.config/time-tracker/
```
