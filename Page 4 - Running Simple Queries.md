Content for Page 4: Running Simple Queries
Great 🚀 Let’s continue with **Page 4: Running Simple Queries** in the Tokio Postgres training guide.

---

# 📘 Page 4: Running Simple Queries

---

## 1. Introduction (Why?)

Once a database connection is established (via single connection or pool), the most common task is **running queries** to read data.

Tokio Postgres provides multiple methods for querying:

* `query` → returns multiple rows.
* `query_one` → expects exactly one row.
* `query_opt` → returns `Some(row)` or `None` (safe for optional results).

👉 Understanding these methods ensures you write **correct, safe, and efficient queries** for different situations.

---

## 2. Syntax / Core Idea

Minimal examples:

```rust
// Returns many rows
let rows = client.query("SELECT id, name FROM users", &[]).await?;

// Returns exactly one row (fails if 0 or >1 rows)
let row = client.query_one("SELECT id, name FROM users WHERE id=$1", &[&1]).await?;

// Returns Option<Row> (None if no match)
let row_opt = client.query_opt("SELECT id, name FROM users WHERE id=$1", &[&999]).await?;
```

---

## 3. Full Real-World Examples

---

### Example 1: Fetch Multiple Rows

```rust
use tokio_postgres::{NoTls, Row};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=testdb", NoTls).await?;
    tokio::spawn(async move { if let Err(e) = connection.await { eprintln!("connection error: {}", e); }});

    // Fetch all users
    let rows: Vec<Row> = client.query("SELECT id, name FROM users", &[]).await?;

    for row in rows {
        let id: i32 = row.get(0);  // by index
        let name: String = row.get("name"); // by column name
        println!("User: {} - {}", id, name);
    }
    Ok(())
}
```

👉 Use `query` when expecting **multiple results**.

---

### Example 2: Fetch a Single Row

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=testdb", NoTls).await?;
    tokio::spawn(async move { let _ = connection.await; });

    // Fetch exactly one row
    let row = client.query_one("SELECT id, name FROM users WHERE id=$1", &[&1]).await?;
    let id: i32 = row.get("id");
    let name: String = row.get("name");

    println!("Found user {}: {}", id, name);
    Ok(())
}
```

👉 Use `query_one` when you **know one row must exist**.

---

### Example 3: Fetch Optional Row

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=testdb", NoTls).await?;
    tokio::spawn(async move { let _ = connection.await; });

    // Fetch optional row
    match client.query_opt("SELECT id, name FROM users WHERE id=$1", &[&999]).await? {
        Some(row) => println!("Found user: {}", row.get::<_, String>("name")),
        None => println!("No user with that ID"),
    }
    Ok(())
}
```

👉 Use `query_opt` when the row may or may not exist.

---

### Example 4: Parameterized Queries (Prevent SQL Injection)

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=testdb", NoTls).await?;
    tokio::spawn(async move { let _ = connection.await; });

    let username = "alice"; // external input (safe!)
    let row = client
        .query_one("SELECT id FROM users WHERE name = $1", &[&username])
        .await?;

    println!("Alice's ID: {}", row.get::<_, i32>(0));
    Ok(())
}
```

👉 Always use **placeholders (`$1, $2, ...`)** to avoid injection risks.

---

### Example 5: Using `?` for Error Handling

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=testdb", NoTls).await?;
    tokio::spawn(async move { let _ = connection.await; });

    // If query fails (wrong column), `?` will bubble up the error
    let row = client.query_one("SELECT wrong_column FROM users LIMIT 1", &[]).await?;
    println!("Result: {}", row.get::<_, String>(0));

    Ok(())
}
```

👉 Use `?` to propagate errors naturally instead of handling manually every time.

---

### Example 6: Typed Query Result Extraction

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=testdb", NoTls).await?;
    tokio::spawn(async move { let _ = connection.await; });

    let row = client.query_one("SELECT id, balance FROM accounts WHERE id=$1", &[&1]).await?;
    let (id, balance): (i32, f64) = (row.get("id"), row.get("balance"));

    println!("Account {} has balance {}", id, balance);
    Ok(())
}
```

👉 Directly extract typed values from rows into Rust variables.

---

## 4. Best Practices & Tips

* ✅ Use `query` only when you expect multiple rows.
* ✅ Use `query_one` when exactly one row must exist (e.g., fetching by primary key).
* ✅ Use `query_opt` for optional results (safer than `query_one`).
* ✅ Always use **parameterized queries** to prevent SQL injection.
* ✅ Use `row.get("column")` with column names for better readability.
* ❌ Don’t build queries by concatenating strings with user input.

---

## 5. Assignments (Practice)

1. Write a query to fetch all users and print them.
2. Write a query that fetches a single user by ID (using `query_one`).
3. Modify it to use `query_opt` and handle the case where the user may not exist.
4. Write a parameterized query to find users whose balance is greater than a given value.
5. Try running a wrong query (invalid column) and see how error handling works.

---

✅ That completes **Page 4 (Running Simple Queries)**.

Would you like me to move on to **Page 5: Insert, Update, Delete Operations** next?
