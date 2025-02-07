use chrono::{Utc};
use chrono_tz::Tz;

fn get_timezone_offset_seconds(timezone: &str) -> Option<i32> {
    let tz: Tz = timezone.parse().ok()?;  // Parse the timezone string
    let now = Utc::now();  // Get current UTC time
    let local_time = now.with_timezone(&tz);
    
    Some(local_time.offset().utc_minus_local())  // Return offset in seconds
}

fn main() {
    let timezone = "Asia/Kolkata";
    
    match get_timezone_offset_seconds(timezone) {
        Some(offset) => println!("Offset for {}: {} seconds", timezone, offset),
        None => println!("Invalid timezone"),
    }
}