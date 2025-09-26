Content for Page 3: Connection Pooling with Deadpool

Perfect 👍 Let’s dive into **Page 3: Connection Pooling with Deadpool**, continuing the detailed training guide format.

---

# 📘 Page 3: Connection Pooling with Deadpool

---

## 1. Introduction (Why?)

In real-world applications, we usually need **many queries at the same time**.
Opening and closing a new PostgreSQL connection for every request is:

* **Slow** (handshake overhead).
* **Resource-heavy** (Postgres has a connection limit).

👉 **Connection pooling** solves this problem:

* Keeps a pool of ready-to-use connections.
* Automatically reuses them for new queries.
* Ensures better **performance** and **scalability**.

We’ll use **`deadpool-postgres`**, a popular pooling library built for async Rust.

---

## 2. Syntax / Core Idea

Minimal setup with Deadpool:

```rust
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a connection manager
    let mgr = Manager::new(
        "host=localhost user=postgres password=postgres dbname=mydb".parse()?,
        NoTls,
    );

    // Build the pool
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();

    // Fetch a client from the pool
    let client = pool.get().await?;

    // Run query
    let row = client.query_one("SELECT 1 + 1", &[]).await?;
    println!("Result: {}", row.get::<_, i32>(0));

    Ok(())
}
```

---

## 3. Full Real-World Examples

---

### Example 1: Basic Pool Usage

```rust
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connection string
    let conn_str = "host=localhost user=postgres password=1234 dbname=testdb";

    // Create pool with max 4 connections
    let mgr = Manager::new(conn_str.parse()?, NoTls);
    let pool = Pool::builder(mgr).max_size(4).build().unwrap();

    // Get a client from the pool
    let client = pool.get().await?;
    let row = client.query_one("SELECT current_database()", &[]).await?;
    println!("Connected to DB: {}", row.get::<_, String>(0));

    Ok(())
}
```

👉 Shows **basic pool setup** and usage.

---

### Example 2: Multiple Queries with Shared Pool

```rust
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::NoTls;
use futures::future;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn_str = "host=localhost user=postgres password=1234 dbname=testdb";
    let mgr = Manager::new(conn_str.parse()?, NoTls);
    let pool = Pool::builder(mgr).max_size(8).build().unwrap();

    // Run multiple queries concurrently using the pool
    let queries = (1..=5).map(|i| {
        let pool = pool.clone();
        tokio::spawn(async move {
            let client = pool.get().await.unwrap();
            let row = client.query_one("SELECT $1::INT * 2", &[&i]).await.unwrap();
            println!("Task {} result: {}", i, row.get::<_, i32>(0));
        })
    });

    future::join_all(queries).await;
    Ok(())
}
```

👉 Demonstrates **concurrent tasks** reusing pooled connections.

---

### Example 3: Pool with Environment Variables

```rust
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::NoTls;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // DATABASE_URL="host=localhost user=postgres password=1234 dbname=testdb"
    let conn_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mgr = Manager::new(conn_str.parse()?, NoTls);
    let pool = Pool::builder(mgr).max_size(10).build().unwrap();

    let client = pool.get().await?;
    let row = client.query_one("SELECT current_date", &[]).await?;
    println!("Today's date: {}", row.get::<_, chrono::NaiveDate>(0));

    Ok(())
}
```

👉 Securely configure pool with **environment variables**.

---

### Example 4: Handling Pool Exhaustion

```rust
use deadpool_postgres::{Manager, Pool, PoolError};
use tokio_postgres::NoTls;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn_str = "host=localhost user=postgres password=1234 dbname=testdb";
    let mgr = Manager::new(conn_str.parse()?, NoTls);
    let pool = Pool::builder(mgr).max_size(1).build().unwrap();

    // First connection occupies the only slot
    let _client1 = pool.get().await?;

    // Trying to get another will block until available
    match tokio::time::timeout(Duration::from_secs(1), pool.get()).await {
        Ok(Ok(_client2)) => println!("Got second connection"),
        Ok(Err(e)) => println!("Pool error: {}", e),
        Err(_) => println!("Timed out waiting for pool connection"),
    }

    sleep(Duration::from_secs(2)).await; // Release client1
    Ok(())
}
```

👉 Shows what happens when the pool **runs out of connections**.

---

### Example 5: Using Pool in a Web Server (Axum)

```rust
use axum::{Router, routing::get, extract::State};
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::NoTls;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let conn_str = "host=localhost user=postgres password=1234 dbname=testdb";
    let mgr = Manager::new(conn_str.parse().unwrap(), NoTls);
    let pool = Pool::builder(mgr).max_size(8).build().unwrap();

    let app = Router::new()
        .route("/", get(handler))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn handler(State(pool): State<Pool>) -> String {
    let client = pool.get().await.unwrap();
    let row = client.query_one("SELECT current_timestamp", &[]).await.unwrap();
    let ts: chrono::NaiveDateTime = row.get(0);
    format!("Server time: {}", ts)
}
```

👉 Example of **using pooled connections in an Axum web API**.

---

## 4. Best Practices & Tips

* ✅ Always configure pool size based on workload (too few = blocking, too many = memory pressure).
* ✅ Use **environment variables** for credentials & connection strings.
* ✅ Handle **pool exhaustion** gracefully (timeouts, retries).
* ✅ Clone the pool freely → it’s cheap; all clones point to the same pool.
* ❌ Don’t keep clients forever; release them back quickly for reuse.
* ✅ For high-throughput services, monitor Postgres max connections and tune accordingly.

---

## 5. Assignments (Practice)

1. Create a pool with max size `2`, then try to run **5 queries concurrently**. Observe blocking behavior.
2. Modify the pool configuration to increase size and rerun — note the difference.
3. Use `DATABASE_URL` from `.env` file to configure the pool.
4. Build a small Axum API that queries a table using a pooled connection.
5. Simulate a heavy load by running `100` queries concurrently and monitor pool performance.

---

✅ That completes **Page 3 (Connection Pooling with Deadpool)**.

Would you like me to move on to **Page 4: Running Simple Queries** next?
