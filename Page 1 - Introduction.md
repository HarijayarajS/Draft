Content for Page 1: Introduction

Great — I’ll start by expanding **Page 1: *Introduction & Overview*** into a fully detailed page following your exact page format (Introduction → Syntax/Core Idea → Full Real-World Examples → Best Practices & Tips → Assignments). Per your rule to produce **one page at a time**, this reply is **Page 1**. It’s written for trainees who already know Rust, but are new to async DB access with **tokio-postgres**.

---

# Page 1 — Introduction & Overview

## 1) Introduction — *Start with the “Why”*

**Why this page matters:**
Databases (like PostgreSQL) are where real applications keep persistent state — users, tasks, orders, logs. `tokio-postgres` is an *asynchronous* PostgreSQL driver for Rust that lets your service handle many concurrent requests without blocking threads while waiting for the database. This unlocking of concurrency is what makes networks-scale web services responsive.

**Analogy:**
Think of your app threads as cooks in a busy kitchen. If a cook must wait by the oven while something bakes (blocking), other orders pile up. `tokio-postgres` hands the cook a pager — the cook can start another dish and come back when the pager buzzes (async), keeping throughput high.

**Real-world problems solved:**

* Efficiently serving many concurrent web requests while performing DB operations.
* Avoiding thread-per-connection explosion.
* Allowing fine-grained control over queries, transactions, and performance tuning.

```
[Visual: connection lifecycle]
Client <------> (Query) <-----> PostgreSQL
   |
 Spawn background task runs the Connection driving I/O (must not be awaited directly on main)
```

---

## 2) Syntax / Core Idea (minimal example)

Minimal pattern to connect and run one query:

```rust
// Cargo.toml (minimal):
// [dependencies]
// tokio = { version = "1", features = ["full"] }
// tokio-postgres = "0.7"

use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect returns (Client, Connection)
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=postgres", NoTls).await?;

    // IMPORTANT: spawn the connection on a task so it drives I/O asynchronously
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Run a simple query
    let row = client.query_one("SELECT version()", &[]).await?;
    let version: &str = row.get(0);
    println!("postgres version: {}", version);

    Ok(())
}
```

**Core idea:** `connect(...)` yields a `Client` (for issuing queries) and a `Connection` future which must be polled (usually by spawning it). Once spawned, `Client` is used to run queries (`query`, `query_one`, `execute`, etc.).

---

## 3) Full Real-World Examples (5–6 complete, runnable examples)

Each example includes thorough inline comments so trainees can copy-paste and run.

> **Before running examples:** either ensure PostgreSQL is running locally (default port 5432) or follow the Docker example below.

---

### Example 1 — Minimal connect & get server version (copy-paste runnable)

```rust
// File: src/main.rs
// Cargo.toml must include tokio and tokio-postgres (see earlier snippet)

use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connection string: host, user, password, dbname (change as needed)
    let conn_str = "host=localhost user=postgres password=postgres dbname=postgres";

    // connect returns: (Client, Connection)
    let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;

    // Spawn connection so it runs in the background and drives socket I/O
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            // If the connection future returns an error, log it
            eprintln!("connection error: {}", e);
        }
    });

    // Run a single-row query and fetch the first row
    let row = client.query_one("SELECT version()", &[]).await?;

    // Row::get(0) returns the first column; type annotated as &str
    let version: &str = row.get(0);
    println!("postgres version: {}", version);

    Ok(())
}
```

**Explanation:**

* Connect string → `connect(...)` → spawn `connection`.
* Use `client.query_one(...)` for queries that must return exactly one row; it errors if 0 or >1 rows. This example is a canonical “hello world” check.

---

### Example 2 — Create table, insert, and query rows (CRUD skeleton)

```rust
// File: src/main.rs
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use a connection string appropriate for your local environment
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=postgres", NoTls).await?;

    // spawn the connection background task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create a table if not exists (DDL)
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS person (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            age INT NOT NULL
        );
        ",
    ).await?;

    // Parameterized INSERT (prevents SQL injection)
    client.execute(
        "INSERT INTO person (name, age) VALUES ($1, $2)",
        &[&"Alice", &30],
    ).await?;

    // Query many rows with filtering using parameters
    let rows = client
        .query("SELECT id, name, age FROM person WHERE age > $1", &[&20])
        .await?;

    // Iterate through result rows and map into Rust types
    for row in rows {
        let id: i32 = row.get(0);       // first column
        let name: &str = row.get(1);    // second column
        let age: i32 = row.get(2);      // third column
        println!("id={} name={} age={}", id, name, age);
    }

    Ok(())
}
```

**Explanation:**

* Uses `batch_execute` to run DDL.
* Uses `execute` for `INSERT`. `execute` returns `u64` rows affected (useful to confirm).
* Uses `query` to retrieve multiple rows; map rows using `row.get(...)`.

---

### Example 3 — `query_one` vs `query_opt` vs `query` (handling 0/1/many)

```rust
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=postgres", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // query_one -> returns Result<Row, Error> and errors if 0 or >1 rows
    match client.query_one("SELECT name FROM person WHERE id = $1", &[&1]).await {
        Ok(row) => {
            let name: &str = row.get(0);
            println!("found (query_one): {}", name);
        }
        Err(e) => eprintln!("query_one error: {}", e),
    }

    // query_opt -> returns Ok(Some(Row)) if 1 row, Ok(None) if 0 rows
    match client.query_opt("SELECT name FROM person WHERE id = $1", &[&999]).await? {
        Some(row) => println!("found (query_opt): {}", row.get::<usize, &str>(0)),
        None => println!("no row found with id 999 (query_opt)"),
    }

    // query -> returns Vec<Row> for any number of rows
    let rows = client.query("SELECT id, name FROM person", &[]).await?;
    println!("found {} rows (query)", rows.len());

    Ok(())
}
```

**Explanation:**

* Use `query_one` when you expect **exactly one** row.
* Use `query_opt` when 0 or 1 rows possible (safer than `query_one`).
* Use `query` when the result may contain many rows.

---

### Example 4 — Connect via `Config` builder and environment variable (recommended for apps)

```rust
use std::env;
use tokio_postgres::{Config, NoTls};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read DATABASE_URL from environment; fallback to a reasonable default
    let conn_str = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "host=localhost user=postgres password=postgres dbname=postgres".into());

    // Parse connection string into Config
    let config: Config = conn_str.parse()?;

    // Connect using config; same pattern: (client, connection)
    let (client, connection) = config.connect(NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // quick check
    let row = client.query_one("SELECT current_database()", &[]).await?;
    let dbname: &str = row.get(0);
    println!("connected to database: {}", dbname);

    Ok(())
}
```

**Explanation:**

* `Config` parsing lets you use a `DATABASE_URL` env var (12-factor config).
* Avoids leaking credentials into source code.

---

### Example 5 — Run PostgreSQL locally with Docker (dev setup)

**Option A — single `docker run`:**

```bash
# Start a local postgres container, exposed on 5432
docker run --name tp-postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:15
```

**Option B — docker-compose.yml (recommended for repeatable dev)**

```yaml
version: '3.8'
services:
  db:
    image: postgres:15
    restart: always
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: postgres
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data
volumes:
  pgdata:
```

**Explanation:**

* This spins up a persistent local PostgreSQL instance bound to `localhost:5432`.
* Use these during development; production uses managed DBs or hardened containers.

---

### Example 6 — Lightweight script showing error handling with `?` and mapping rows to a struct

```rust
use tokio_postgres::NoTls;

// Simple struct for mapping
struct Person {
    id: i32,
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=postgres",
        NoTls,
    ).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Query rows and manually map into Person structs
    let rows = client.query("SELECT id, name, age FROM person ORDER BY id LIMIT 10", &[]).await?;
    let people: Vec<Person> = rows.into_iter().map(|row| {
        Person {
            id: row.get(0),
            name: row.get::<usize, &str>(1).to_string(),
            age: row.get(2),
        }
    }).collect();

    for p in people {
        println!("{}: {} ({})", p.id, p.name, p.age);
    }

    Ok(())
}
```

**Explanation:**

* Demonstrates idiomatic mapping from `Row` to a Rust struct (manual `FromRow` pattern can be added later).
* Uses `?` to bubble up errors for concise handling.

---

## 4) Best Practices & Tips (practical do’s & don’ts)

**Do:**

* **Always spawn the `Connection` future** returned by `connect()` (it drives I/O). Failure to do so will hang queries.
* **Use parameterized queries** (`$1`, `$2`) and pass parameters via `&[ &value ]` to avoid SQL injection.
* **Keep credentials in env vars / secrets manager**, not in code. Use `DATABASE_URL` or config files injected at runtime.
* **Use connection pooling** (e.g., `deadpool-postgres`) in web servers — creating new connections per request is slow and resource-heavy. (Pooling page follows.)
* **Run migrations** to manage schema evolution (tools like `sqlx` migrations, `refinery`, or plain SQL migration scripts).
* **Use small transactions** and commit/rollback as soon as possible — avoid long-lived open transactions.

**Don’t:**

* Don’t hold a `Client` while doing CPU-bound work that blocks the tokio executor — spawn blocking or use `tokio::task::spawn_blocking` for heavy CPU tasks.
* Don’t store raw passwords in source control.
* Don’t use `query_one` when 0 rows is possible — prefer `query_opt` and handle `None`.
* Don’t open many DB connections in production without pooling limits — set sensible maximum connections to avoid overwhelming Postgres.

**Performance notes:**

* For many small inserts, prefer **prepared statements** or batch inserts to reduce overhead.
* In high-throughput systems, watch for **connection saturation** — tune pool size depending on Postgres configuration (max_connections) and application concurrency.

**Security:**

* Use TLS (e.g., `tokio-postgres` with `native-tls`/`rustls` connector) for production connections.
* Grant least-privilege DB users — separate user for app and admin activities.

---

## 5) Assignments (Practice) — progressively harder (5+ tasks)

1. **Dev setup & connect**

   * Task: Run Postgres locally with Docker (`docker run` or `docker-compose`) and run Example 1 to print Postgres version.
   * Goal: Validate connectivity from your machine.

2. **Create + Query**

   * Task: Implement Example 2 fully: create `person` table, insert five rows with different ages, then query and print people older than 25.
   * Goal: Learn DDL + DML basics and parameterized queries.

3. **Handle optional results**

   * Task: Write a function `find_person_by_name(client: &Client, name: &str) -> Result<Option<(i32, String)>, Error>` that uses `query_opt`. Use it with existing data.
   * Goal: Practise `query_opt` and `Option` handling.

4. **Config & env**

   * Task: Change your program to read `DATABASE_URL` from environment and parse it via `Config`. Add helpful error messages if env var missing.
   * Goal: Learn 12-factor style config.

5. **Basic mapping & CLI**

   * Task: Build a small CLI (using `clap` optional) that supports commands `list`, `add <name> <age>`, `find <name>`. Use a `Person` struct and manual mapping.
   * Goal: Practical integration with an app-style workflow.

6. **Prepare for pooling (lead-in)**

   * Task: Refactor one of the examples into an `async fn run(db_conn_str: &str)` and separate connection setup from business logic. Add a TODO: replace single connection with `deadpool` pool in next page.
   * Goal: Understand code separation and prepare for pooling.

---

## Quick Summary / Checklist (what you should now know)

* What `tokio-postgres` is and why async DB access matters.
* How to connect (`connect`) and why you must spawn the `Connection` background task.
* How to run simple queries (`query`, `query_one`, `query_opt`, `execute`).
* How to create a table, insert rows, and map rows to Rust types.
* How to run Postgres locally with Docker for development.
* Basic do’s & don’ts and practice exercises to cement skills.

---

If this looks good, I’ll continue with **Page 2: *Establishing a Connection*** (deep dive into `Client` vs `Connection`, lifecycle, advanced connection patterns, error cases, and many hands-on examples like TLS and `Config` builder variations). Want me to proceed with Page 2 now?
