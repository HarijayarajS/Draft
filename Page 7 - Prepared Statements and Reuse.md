Content for Page 7: Prepared Statements and Reuse

Perfect! Let’s continue with **Page 7: Prepared Statements and Reuse** in the Tokio Postgres training guide. This page teaches trainees how to **improve performance** and **write reusable queries**.

---

# 📘 Page 7: Prepared Statements and Reuse

---

## 1. Introduction (Why?)

In real-world applications, you often run **the same query multiple times** (e.g., inserting many rows, fetching users by ID).

* Preparing queries **once** and reusing them is more efficient than sending raw SQL each time.
* Benefits of prepared statements:

  * **Performance:** Query plan is prepared once by PostgreSQL.
  * **Security:** Avoids SQL injection.
  * **Convenience:** Reusable across multiple calls.

Tokio Postgres provides `prepare` and `prepare_typed` for this purpose.

---

## 2. Syntax / Core Idea

```rust
// Prepare a query
let stmt = client.prepare("INSERT INTO users (name, email) VALUES ($1, $2)").await?;

// Reuse the prepared statement multiple times
client.execute(&stmt, &[&"Alice", &"alice@example.com"]).await?;
client.execute(&stmt, &[&"Bob", &"bob@example.com"]).await?;
```

* `$1, $2` → placeholders for parameters
* `&stmt` → reference to the prepared statement

---

## 3. Full Real-World Examples

---

### Example 1: Simple Reuse

```rust
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=mydb", NoTls).await?;
    tokio::spawn(async move { let _ = connection.await; });

    // Prepare once
    let stmt = client.prepare("INSERT INTO users (name, email) VALUES ($1, $2)").await?;

    // Reuse multiple times
    let users = vec![
        ("Alice", "alice@example.com"),
        ("Bob", "bob@example.com"),
        ("Charlie", "charlie@example.com"),
    ];

    for (name, email) in users {
        client.execute(&stmt, &[&name, &email]).await?;
    }

    println!("Inserted multiple users using prepared statement!");
    Ok(())
}
```

✅ Shows **performance and code reuse** in batch inserts.

---

### Example 2: Typed Prepared Statements

```rust
// Prepare typed query
let stmt = client.prepare_typed(
    "INSERT INTO accounts (id, balance) VALUES ($1, $2)",
    &[&i32, &f64] // types of parameters
).await?;

client.execute(&stmt, &[&1, &1000.0]).await?;
client.execute(&stmt, &[&2, &500.0]).await?;
```

* Helps **compile-time type safety** and prevents runtime errors.

---

### Example 3: Reuse for SELECT Queries

```rust
let select_stmt = client.prepare("SELECT id, name FROM users WHERE id = $1").await?;

for id in 1..4 {
    let row = client.query_one(&select_stmt, &[&id]).await?;
    println!("User {}: {}", row.get::<_, i32>("id"), row.get::<_, String>("name"));
}
```

* Efficient when fetching **multiple users by ID repeatedly**.

---

### Example 4: Using Transactions with Prepared Statements

```rust
let txn = client.transaction().await?;
let stmt = txn.prepare("INSERT INTO logs (message) VALUES ($1)").await?;

txn.execute(&stmt, &[&"Starting transaction"]).await?;
txn.execute(&stmt, &[&"Processing data"]).await?;
txn.commit().await?;
```

* Can **reuse prepared statements inside transactions** for multiple operations.

---

### Example 5: Dynamic Reuse in Loops

```rust
let stmt = client.prepare("UPDATE accounts SET balance = $1 WHERE id = $2").await?;
let updates = vec![(1200.0, 1), (800.0, 2), (1500.0, 3)];

for (balance, id) in updates {
    client.execute(&stmt, &[&balance, &id]).await?;
}
println!("Updated multiple accounts efficiently!");
```

* Reduces query parsing overhead when **looping over multiple updates**.

---

### Example 6: Combining Prepared Statements & Optional Parameters

```rust
let stmt = client.prepare("SELECT id, name FROM users WHERE email = $1").await?;
let emails = vec!["alice@example.com", "bob@example.com", "unknown@example.com"];

for email in emails {
    match client.query_opt(&stmt, &[&email]).await? {
        Some(row) => println!("Found user: {}", row.get::<_, String>("name")),
        None => println!("No user with email {}", email),
    }
}
```

* Shows **safe handling of optional results** with prepared statements.

---

## 4. Best Practices & Tips

* ✅ Prepare queries **once** and reuse wherever possible.
* ✅ Use **typed prepared statements** for safety.
* ✅ Combine with transactions for batch operations.
* ✅ Avoid preparing queries inside loops repeatedly → prepare **outside the loop**.
* ❌ Do not concatenate raw strings for queries → always use `$1, $2` placeholders.
* ✅ Consider caching prepared statements if used frequently in long-lived applications.

---

## 5. Assignments (Practice)

1. Prepare an INSERT statement and insert 5+ users using a loop.
2. Prepare a SELECT query and fetch multiple users by ID using the same prepared statement.
3. Use `prepare_typed` to insert numeric data safely.
4. Reuse a prepared UPDATE statement to modify multiple rows in a loop.
5. Combine prepared statements with a transaction for batch inserts/updates.

---

✅ That completes **Page 7: Prepared Statements and Reuse**.

Do you want me to continue with **Page 8: Mapping Rows to Rust Structs (Manual ORMs)** next?




Absolutely! Let’s continue with **Page 8: Mapping Rows to Rust Structs** in the Tokio Postgres guide. This page is critical for helping trainees **convert raw database rows into structured Rust data types** for easier use.

---