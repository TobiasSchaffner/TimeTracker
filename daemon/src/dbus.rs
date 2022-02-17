use dbus::blocking::Connection;
use std::time::Duration;

const DBUS_SCREENSAVERS: &[&str] = &[
    "org.gnome.ScreenSaver",
    "org.cinnamon.ScreenSaver",
    "org.kde.screensaver",
    "org.freedesktop.ScreenSaver",
];

pub fn is_screensaver_active() -> bool {
    // First open up a connection to the session bus.
    let conn = Connection::new_session().expect("Dbus connection failed!");

    for screensaver in DBUS_SCREENSAVERS {
        let proxy = conn.with_proxy(
            *screensaver,
            format!("/{}", str::replace(*screensaver, ".", "/")),
            Duration::from_millis(5000),
        );
        match proxy.method_call(*screensaver, "GetActive", ()) {
            Ok((active,)) => return active,
            Err(_) => (),
        };
    }

    panic!("Screensaver not supported!");
}
