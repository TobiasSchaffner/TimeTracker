use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use rand::Rng;

fn draw(drawing_area: &DrawingArea) {
    drawing_area.connect_draw(move |w, c| {
        println!("w: {} c:{}", w, c);
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0.0..1.0);
        let g = rng.gen_range(0.0..1.0);
        let b = rng.gen_range(0.0..1.0);
        c.set_source_rgb(r, g, b);
        c.rectangle(1.0, 1.0, 100.0, 200.0);
        c.fill().expect("");
        gtk::Inhibit(false)
    });
}

fn app(app:&gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("TimeTracker")
        .default_width(320)
        .default_height(280)
        .build();

    let frame = gtk::Frame::new(None);
    let area = DrawingArea::new();

    draw(&area);

    frame.add(&area);
    window.add(&frame);
    window.show_all();
}

fn main() {
    let application = Application::builder()
        .application_id("Time Tracker")
        .build();
    application.connect_activate(app);
    application.run();
}
