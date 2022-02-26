use clap::Parser;

use ttlib;

fn print_date_offset(offset: i64, verbose: bool) {
    let date = ttlib::time::get_date(offset);
    let config = ttlib::config::load_config(date);

    let mut sum: i64 = 0;
    let last = config.activities_timestamp.len() - 1;

    for i in 0..config.activities_duration.len() {
        sum += config.activities_duration[i];
    }

    let start_time = ttlib::time::local_time_from_millis(config.activities_timestamp[0]);
    print!(
        "{} - start: {}",
        date.format("%a, %v"),
        start_time.format("%H:%M"),
    );

    if offset != 0 {
        let end_time = ttlib::time::local_time_from_millis(
            config.activities_timestamp[last] + config.activities_duration[last],
        );
        print!(" - end: {}", end_time.format("%H:%M"));
    }

    let total_time = ttlib::time::duration_from_millis(sum);

    print!(
        " - total: {:02}:{:02}",
        total_time.num_hours(),
        total_time.num_minutes() % 60
    );

    let total_break = ttlib::time::duration_from_millis(
        ((config.activities_timestamp[last] + config.activities_duration[last])
            - config.activities_timestamp[0])
            - sum,
    );

    print!(
        " - break: {:02}:{:02}",
        total_break.num_hours(),
        total_break.num_minutes() % 60
    );

    println!();

    if verbose && config.activities_timestamp.len() > 1 {
        for i in 0..config.activities_timestamp.len() - 2 {
            let start_break = ttlib::time::local_time_from_millis(
                config.activities_timestamp[i] + config.activities_duration[i],
            );
            let end_break = ttlib::time::local_time_from_millis(
                config.activities_timestamp[i + 1],
            );
            let duration = ttlib::time::duration_from_millis(
                config.activities_timestamp[i + 1]
                    - (config.activities_timestamp[i] + config.activities_duration[i]),
            );
            println!(
                "    start: {} - end: {} - total: {:02}:{:02}:{:02}",
                start_break.format("%H:%M"),
                end_break.format("%H:%M"),
                duration.num_hours(),
                duration.num_minutes() % 60,
                duration.num_seconds() % 60,
            );
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The start of the printed range
    #[clap(short, long, allow_hyphen_values = true, default_value_t = -7)]
    start: i64,

    /// The number of dates that should be printed
    #[clap(short, long, allow_hyphen_values = true, default_value_t = 7)]
    number: i64,

    /// Print all breaks
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    for i in args.start + 1..args.start + args.number + 1 {
        print_date_offset(i, args.verbose);
    }
}
