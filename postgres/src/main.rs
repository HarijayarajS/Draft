use chrono::{DateTime, Datelike, NaiveDate, Utc};
use tokio_postgres::{NoTls, Error};

#[tokio::main] 
async fn main() -> Result<(), Error> {
    // Configure the connection to the PostgreSQL database
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres dbname=postgres password=password",
        NoTls,
    )
    .await?;

    // Spawn a task to process the connection asynchronously
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
        
    fn convert_naive_date_to_datetime(naive_date: NaiveDate) -> DateTime<Utc> {
        DateTime::<Utc>::from_naive_utc_and_offset(naive_date.and_hms_opt(0, 0, 0).unwrap(), Utc)
    }

    let employee_id = 1002;
    let start = NaiveDate::from_ymd_opt(2024,03,1).unwrap();
    let end = NaiveDate::from_ymd_opt(2024,03,29).unwrap();

    let start_date_db = convert_naive_date_to_datetime(NaiveDate::from_ymd_opt(2024,03,1).unwrap());
    let end_date_db = convert_naive_date_to_datetime(NaiveDate::from_ymd_opt(2024,03,29).unwrap());

    #[derive(Debug)]
    struct StatusLogEmployeeItem{
        date: NaiveDate,
        in_time: String,
        out_time:  String,
        break_time:  i32,
        lunch_time:  i32,
        day_type:  Option<String>,
    }

    // Perform a simple query to select some data from a table
    let rows = client
        .query("
        SELECT 
        DATE(created_on) AS created_date,
        SUBSTRING(created_on::TEXT, 12, 8) AS created_time,
        SUBSTRING(modified_on::TEXT, 12, 8) AS modified_time,
        employee_id, 
        CAST(SUM(CASE WHEN status = 'break-in' THEN time_taken ELSE 0 END) AS INT) AS break_time, 
        CAST(SUM(CASE WHEN status = 'Lunch-in' THEN time_taken ELSE 0 END) AS INT) AS lunch_time 
    FROM 
        employee_status_log 
    WHERE 
        created_on >= $1 
        AND created_on <= $2 
      GROUP BY 
      DATE(created_on), 
      SUBSTRING(created_on::TEXT, 12, 8),
      SUBSTRING(modified_on::TEXT, 12, 8),
      employee_id
      ", &[&start_date_db,&end_date_db])
        .await?;

    // Iterate over the rows and print them

    let mut items: Vec<StatusLogEmployeeItem> =  vec![];

    for date in start.iter_days().take_while(|&d| d <= end) {
        let mut found = false;
        for row in &rows {
            if row.get::<_, NaiveDate>("created_date") == date {
                found = true;
                items.push(StatusLogEmployeeItem {
                    date: row.get("created_date"),
                    in_time: row.get("created_time"),
                    out_time: row.get("modified_time"),
                    break_time: row.get("break_time"),
                    lunch_time: row.get("lunch_time"),
                    day_type: Some("Present".to_string()),
                });
                break;
            }
        }
        if !found {
            // If no data for this date, consider it as leave
            items.push(StatusLogEmployeeItem {
                date,
                in_time: "".to_string(),
                out_time: "".to_string(),
                break_time: 0,
                lunch_time: 0,
                day_type: Some("Leave".to_string()),
            });
        }
    }

    // Mark Sundays as holidays
    for item in &mut items {
        if item.date.weekday() == chrono::Weekday::Sun {
            item.day_type = Some("Holiday".to_string());
        }
    }

    // for row in &rows {
    //     let date: NaiveDate = row.get("created_date");
    //     if date.weekday().clone().num_days_from_monday()==1{
    //         println!("{:?}",date.weekday());
    //     }
    //     items.push(StatusLogEmployeeItem{
    //          date: row.get("created_date"), 
    //          in_time: row.get("created_time"), 
    //          out_time: row.get("modified_time"), 
    //          break_time:row.get("break_time"), 
    //          lunch_time: row.get("lunch_time"), 
    //          day_type: Some("new".to_string()) 
    //         });
    // }
    println!("{:#?}", items);
    Ok(())
}
