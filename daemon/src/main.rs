extern crate dirs;
use std::{thread, time};
use ttlib;

mod dbus;


const UPDATE_INTERVALL_MILLIS: u64 = 1000;


fn adjust_screen_time(config: &mut ttlib::config::Config, init: bool) {
    let last_timestamp = config
        .activities_timestamp
        .last_mut()
        .expect("Empty timestamp vector!");

    let last_duration = config
        .activities_duration
        .last_mut()
        .expect("Empty duration vector!");

    if init || dbus::is_screensaver_active() {
        if *last_duration != 0 {
            config.activities_timestamp.push(ttlib::time::current_time());
            config.activities_duration.push(0);
            ttlib::config::save_config(config);
        }
    } else {
        if *last_duration == 0 {
            *last_timestamp = ttlib::time::current_time();
            *last_duration = UPDATE_INTERVALL_MILLIS as i64;
        } else {
            *last_duration = ttlib::time::current_time() - *last_timestamp;
        }
        ttlib::config::save_config(config);
    }
}

fn main() {
    let mut init = true;
    loop {
        adjust_screen_time(&mut ttlib::config::load_config(ttlib::time::current_date()), init);
        thread::sleep(time::Duration::from_millis(UPDATE_INTERVALL_MILLIS));
        init = false;
    }
}
