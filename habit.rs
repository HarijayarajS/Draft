use chrono::Local;
use dialoguer::Select;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, time::Duration};
use tokio::time::interval;

#[derive(Serialize, Deserialize, Debug)]
struct HabitEntry {
    timestamp: String,
    did_it: bool,
}

async fn ask_and_store(path: &PathBuf) {
    // Send system notification
    Notification::new()
        .summary("Habit Check")
        .body("Did you do it?")
        .show()
        .unwrap();

    // Small delay to let user notice the notification
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Terminal-based answer
    let options = &["Yes", "No"];
    let selection = Select::new()
        .with_prompt("Did you do it?")
        .default(1)
        .items(options)
        .interact()
        .unwrap();

    let entry = HabitEntry {
        timestamp: Local::now().to_rfc3339(),
        did_it: selection == 0,
    };

    let mut history = if path.exists() {
        let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".into());
        serde_json::from_str::<Vec<HabitEntry>>(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    history.push(entry);
    fs::write(path, serde_json::to_string_pretty(&history).unwrap()).unwrap();
}

#[tokio::main]
async fn main() {
    let path = PathBuf::from("habit_log.json");
    let mut interval = interval(Duration::from_secs(60 * 60)); // every 1 hour

    loop {
        interval.tick().await;
        ask_and_store(&path).await;
    }
}


[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
notify-rust = "4"
dialoguer = "0.11"
chrono = "0.4"

