Content for Page 2: Single Connection


Great 👍 Let’s continue with **Page 2: Establishing a Connection** in full detail, following the training guide format we set earlier.

---

# 📘 Page 2: Establishing a Connection in Tokio Postgres

---

## 1. Introduction (Why?)

When building a Rust application that interacts with PostgreSQL, the very first step is **establishing a connection**.
In Tokio Postgres, two important actors exist:

* **`Client`** → Used to run SQL queries (like sending commands to the DB).
* **`Connection`** → Manages the actual link to PostgreSQL and must run in the background on a Tokio task.

If you don’t spawn the `Connection` on a task, queries will **hang forever**, because no one is driving the communication with the database.

Think of it like this:

* The `Client` is the **driver** giving instructions.
* The `Connection` is the **engine** that makes the car actually move. Both are required.

---

## 2. Syntax / Core Idea

Minimal connection setup:

```rust
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the database
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=mydb", NoTls).await?;

    // Spawn the connection so it runs in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Use the client to run queries
    let rows = client.query("SELECT 1 + 1", &[]).await?;
    println!("Result: {}", rows[0].get::<_, i32>(0));

    Ok(())
}
```

Key things to note:

* `connect` returns `(Client, Connection)`.
* Always `tokio::spawn` the `Connection`.
* Use `client` to execute queries.

---

## 3. Full Real-World Examples

Here are several scenarios with explanations:

---

### Example 1: Basic Connection

```rust
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect with simple credentials
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=testdb", NoTls).await?;

    // Run connection in background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("✅ Connected to PostgreSQL!");
    Ok(())
}
```

👉 Demonstrates the simplest working connection.

---

### Example 2: Running a Query Immediately

```rust
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=testdb", NoTls).await?;

    tokio::spawn(async move { connection.await.unwrap() });

    // Run a query
    let row = client.query_one("SELECT current_date", &[]).await?;
    let today: chrono::NaiveDate = row.get(0);
    println!("📅 Today's date is: {}", today);

    Ok(())
}
```

👉 Shows how to run a **query right after connecting**.

---

### Example 3: Using Environment Variables

```rust
use tokio_postgres::{NoTls, Error};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Example: DATABASE_URL="host=localhost user=postgres password=1234 dbname=testdb"
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;
    tokio::spawn(async move { connection.await.unwrap() });

    println!("Connected using DATABASE_URL: {}", db_url);
    Ok(())
}
```

👉 Good practice: store DB credentials in environment variables.

---

### Example 4: Handling Connection Errors

```rust
use tokio_postgres::{NoTls};

#[tokio::main]
async fn main() {
    let conn_str = "host=localhost user=postgres password=wrongpass dbname=testdb";

    match tokio_postgres::connect(conn_str, NoTls).await {
        Ok((client, connection)) => {
            tokio::spawn(async move { connection.await.unwrap() });
            println!("Connected successfully!");
        }
        Err(e) => {
            eprintln!("❌ Failed to connect: {}", e);
        }
    }
}
```

👉 Demonstrates **graceful error handling**.

---

### Example 5: Multiple Connections

```rust
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn_str = "host=localhost user=postgres password=1234 dbname=testdb";

    // Open two independent connections
    for i in 1..=2 {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
        tokio::spawn(async move { connection.await.unwrap() });
        println!("🔌 Connection {} established!", i);

        // Run a test query
        let row = client.query_one("SELECT $1::TEXT", &[&format!("Conn {}", i)]).await?;
        println!("Result from {}: {}", i, row.get::<_, String>(0));
    }

    Ok(())
}
```

👉 Demonstrates running **multiple clients simultaneously**.

---

### Example 6: Connection with SSL

```rust
use tokio_postgres::{Error, tls::NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn_str = "host=myserver.com user=postgres password=secret dbname=securedb sslmode=require";

    let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
    tokio::spawn(async move { connection.await.unwrap() });

    println!("🔒 Secure connection established!");
    Ok(())
}
```

👉 Example of connecting to **remote PostgreSQL with SSL**.

---

## 4. Best Practices & Tips

* Always `tokio::spawn` the `Connection`, otherwise queries won’t progress.
* Use environment variables or config files for credentials (never hardcode passwords).
* Reuse connections wisely → later we’ll cover **connection pools**.
* Always handle errors gracefully (failed connection, invalid credentials, etc.).
* For production, prefer **SSL/TLS connections** to secure DB traffic.

---

## 5. Assignments (Practice)

1. Write a program that connects to a PostgreSQL database and prints the database version.
2. Modify the connection string to use environment variables instead of hardcoding credentials.
3. Simulate a failed connection (wrong password) and handle the error gracefully.
4. Create a program that opens **three connections simultaneously** and runs different queries on each.
5. Connect to PostgreSQL using **SSL mode** and verify that the connection is secure.

---

✅ That completes **Page 2 (Establishing a Connection)**.

Would you like me to move on to **Page 3: Connection Pooling with Deadpool** next?
