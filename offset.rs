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



use chrono::{Utc, TimeZone};
use chrono_tz::Tz;

fn get_timezone_offset_seconds(tz_str: &str) -> Result<i32, String> {
    let tz: Tz = tz_str.parse().map_err(|_| format!("Invalid timezone: {}", tz_str))?;
    let now = Utc::now();
    let offset = tz.offset_from_utc_datetime(&now.naive_utc());
    Ok(offset.local_minus_utc())
}

fn main() {
    match get_timezone_offset_seconds("America/New_York") {
        Ok(offset) => println!("Current offset in seconds: {}", offset),
        Err(e) => eprintln!("Error: {}", e),
    }
}




use chrono::{Utc, TimeZone};
use chrono_tz::Tz;

fn get_timezone_offset_seconds(tz_str: &str) -> Result<i32, String> {
    // Parse the timezone string into a Tz type
    let tz: Tz = tz_str.parse().map_err(|_| format!("Invalid timezone: {}", tz_str))?;

    // Get the current UTC time
    let now = Utc::now();

    // Convert the UTC time to the specified timezone
    let local_time = tz.from_utc_datetime(&now.naive_utc());

    // Get the offset in seconds
    let offset = local_time.offset().fix().local_minus_utc();

    Ok(offset)
}

fn main() {
    match get_timezone_offset_seconds("America/New_York") {
        Ok(offset) => println!("Current offset in seconds: {}", offset),
        Err(e) => eprintln!("Error: {}", e),
    }
}