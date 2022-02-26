use clap::Parser;

use ttlib;

fn print_date_offset(offset: i64) {
    let date = ttlib::time::get_date(offset);
    let config = ttlib::config::load_config(date);

    let mut sum: i64 = 0;

    for length in config.activities_duration {
        sum += length;
    }
    let first_time = ttlib::time::local_time_from_millis(config.activities_timestamp[0]);
    let total_time = ttlib::time::duration_from_millis(sum);
    println!(
        "{} - start: {} - total: {:02}:{:02}",
        date.format("%a, %v"),
        first_time.format("%H:%M"),
        total_time.num_hours(),
        total_time.num_minutes() % 60
    );
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
}

fn main() {
    let args = Args::parse();

    for i in args.start + 1..args.start + args.number + 1 {
        print_date_offset(i);
    }
}
