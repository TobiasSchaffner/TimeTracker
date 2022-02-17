
use ttlib;

fn print_date_offset(offset: i64) {
    let date = ttlib::time::get_date(offset);
    let config = ttlib::config::load_config(date);

    let mut sum: i64 = 0;

    for length in config.activities_duration {
        sum += length;
    }
    // let first_time = ttlib::time::time_from_millis(sum);
    let total_time = ttlib::time::duration_from_millis(sum);
    println!("{} - {:02}:{:02}", date.format("%a, %v"), total_time.num_hours(), total_time.num_minutes() % 60);
}

fn main() {
    for i in -7..1 {
        print_date_offset(i);
    }
}