use chrono::NaiveDate;
fn main() {

    fn get_date_range(date_range: &str) -> Result<(NaiveDate, NaiveDate), &str> {
        let date: Vec<&str> = date_range.split(" - ").collect();
        if date.len() != 2 {
            return Err("Invalid date range format");
        }
        let start_date = NaiveDate::parse_from_str(date[0], "%Y-%m-%d");
        let end_date = NaiveDate::parse_from_str(date[1], "%Y-%m-%d");
    
        if let (Ok(start), Ok(end)) = (start_date, end_date) {
            Ok((start, end))
        } else {
            Err("Unable to parse dates")
        }
    }
    println!("{:?}", get_date_range("2024-03-25 - 2024-03-29"));
}
