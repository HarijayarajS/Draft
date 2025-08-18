use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Entry, Button, Box as GtkBox, Orientation};
use notify_rust::Notification;
use tray_item::TrayItem;
use std::fs::OpenOptions;
use std::io::Write;
use tokio::time::{sleep, Duration};

fn save_log(text: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("hourly_log.txt")
        .unwrap();
    writeln!(file, "{} - {}", chrono::Local::now(), text).unwrap();
}

fn open_input_window(app: &Application) {
    let entry = Entry::new();
    let button = Button::with_label("Save");

    let vbox = GtkBox::new(Orientation::Vertical, 5);
    vbox.append(&entry);
    vbox.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hourly Log")
        .default_width(300)
        .default_height(100)
        .child(&vbox)
        .build();

    let win_clone = window.clone();
    button.connect_clicked(move |_| {
        let text = entry.text().to_string();
        if !text.is_empty() {
            save_log(&text);
        }
        win_clone.close();
    });

    window.show();
}

#[tokio::main]
async fn main() {
    let app = Application::builder()
        .application_id("com.example.HourlyLog")
        .build();

    // System tray
    let mut tray = TrayItem::new("Hourly Log", "edit").unwrap();
    let app_clone = app.clone();
    tray.add_menu_item("Add Note Now", move || {
        open_input_window(&app_clone);
    }).unwrap();

    tray.add_menu_item("Quit", || {
        std::process::exit(0);
    }).unwrap();

    // Hourly reminder loop
    tokio::spawn({
        let app_clone = app.clone();
        async move {
            loop {
                Notification::new()
                    .summary("Hourly Log Reminder")
                    .body("What did you do in the last hour?")
                    .show()
                    .unwrap();

                // Open input window
                open_input_window(&app_clone);

                sleep(Duration::from_secs(3600)).await;
            }
        }
    });

    app.connect_activate(move |app| {
        // Empty window (hidden, app lives in tray)
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hourly Log Background")
            .default_width(1)
            .default_height(1)
            .build();
        window.hide(); // keep hidden
    });

    app.run();
}


[dependencies]
notify-rust = "4"
gtk = { version = "0.9", package = "gtk4" }
tray-item = "0.6"
chrono = "0.4"
tokio = { version = "1", features = ["full"] }