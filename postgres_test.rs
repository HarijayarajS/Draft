use tokio_postgres::{NoTls, Row};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to database
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=postgres",
        NoTls,
    ).await?;

    // Spawn connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Prepare table
    client.execute(
        "DROP TABLE IF EXISTS items",
        &[]
    ).await?;
    client.execute(
        "CREATE TABLE items (id SERIAL PRIMARY KEY, name TEXT NOT NULL)",
        &[]
    ).await?;

    // Insert test data
    let item1 = Item { id: 1, name: "Apple".to_string() };
    let item2 = Item { id: 2, name: "Banana".to_string() };

    client.execute(
        "INSERT INTO items (id, name) VALUES ($1, $2)",
        &[&item1.id, &item1.name]
    ).await?;

    client.execute(
        "INSERT INTO items (id, name) VALUES ($1, $2)",
        &[&item2.id, &item2.name]
    ).await?;

    // Now, check in database
    check_db_insert(&client, &[item1, item2]).await?;

    println!("All items match successfully!");

    Ok(())
}

// --------------- CORE LOGIC ---------------

#[async_trait]
pub trait DbCheckable: Sized + Serialize + DeserializeOwned {
    fn id_value(&self) -> &(dyn tokio_postgres::types::ToSql + Sync);
    fn table_name() -> &'static str;
    fn id_column() -> &'static str;
}

pub async fn check_db_insert<T>(
    client: &tokio_postgres::Client,
    items: &[T],
) -> Result<(), Box<dyn std::error::Error>>
where
    T: DbCheckable + std::fmt::Debug,
{
    for item in items {
        let query = format!("SELECT * FROM {} WHERE {} = $1", T::table_name(), T::id_column());
        let row = client.query_one(&query, &[item.id_value()]).await?;

        let db_item: T = deserialize_row(&row)?;

        if serde_json::to_value(item)? != serde_json::to_value(&db_item)? {
            return Err(format!("Mismatch:\nexpected: {:?}\nfound: {:?}", item, db_item).into());
        }
    }
    Ok(())
}

fn deserialize_row<T: DeserializeOwned>(row: &Row) -> Result<T, Box<dyn std::error::Error>> {
    let mut map = serde_json::Map::new();

    for column in row.columns() {
        let name = column.name();
        let value: Value = match *column.type_() {
            tokio_postgres::types::Type::INT4 => {
                let v: i32 = row.get(name);
                Value::Number(v.into())
            }
            tokio_postgres::types::Type::TEXT | tokio_postgres::types::Type::VARCHAR => {
                let v: String = row.get(name);
                Value::String(v)
            }
            _ => {
                return Err(format!("Unsupported column type: {}", column.type_()).into());
            }
        };
        map.insert(name.to_string(), value);
    }

    Ok(serde_json::from_value(Value::Object(map))?)
}

// --------------- YOUR STRUCT ---------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    id: i32,
    name: String,
}

#[async_trait]
impl DbCheckable for Item {
    fn id_value(&self) -> &(dyn tokio_postgres::types::ToSql + Sync) {
        &self.id
    }

    fn table_name() -> &'static str {
        "items"
    }

    fn id_column() -> &'static str {
        "id"
    }
}




[package]
name = "db_check_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-postgres = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
async-trait = "0.1"