# Time Tracker

A plain simple screen time tracker for linux written in Rust.

It tracks the time that your screen was unlocked.

The tool was created as the other solutions available e.g. activity watch are
too sophisticated for my usecase.

## Usage

Use the time-tracker cli tool to show the total active times per day.

When started without arguments the tool will show the last seven days.

```
$ time-tracker -h
USAGE:
    time-tracker [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -n, --number <NUMBER>    The number of dates that should be printed [default: 7]
    -s, --start <START>      The start of the printed range [default: -7]
    -v, --verbose            Print all breaks
    -V, --version            Print version information
```

Sample output:

```
$ time-tracker -s -5
Mon, 28-Feb-2022 - start: 08:07 - end: 17:51 - total: 09:00 - break: 00:43
Tue,  1-Mar-2022 - start: 08:35 - end: 18:34 - total: 07:48 - break: 02:10
Wed,  2-Mar-2022 - start: 08:08 - end: 18:56 - total: 09:10 - break: 01:38
Thu,  3-Mar-2022 - start: 07:59 - end: 19:56 - total: 09:49 - break: 02:07
Fri,  4-Mar-2022 - start: 08:43 - total: 03:33 - break: 00:45
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

Use run.sh to build the project, install the .deb and run time-tracker.

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
