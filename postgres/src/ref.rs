use chrono::{NaiveDate, NaiveDateTime};

fn convert_naive_date_to_naive_datetime(date: NaiveDate) -> Option<NaiveDateTime> {
    date.and_hms_opt(0, 0, 0)
}

fn main() {
    let date = NaiveDate::from_ymd(2024, 3, 26);
    match convert_naive_date_to_naive_datetime(date) {
        Some(datetime) => println!("NaiveDateTime: {}", datetime),
        None => println!("Failed to convert NaiveDate to NaiveDateTime"),
    }
}

