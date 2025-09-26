Content for Page 9: Handling Common Errors and Safe Retrying

Absolutely! Let’s move on to **Page 9: Handling Common Errors and Safe Retrying**. This is crucial for trainees to **write robust and production-ready applications** using Tokio Postgres.

---

# 📘 Page 9: Handling Common Errors and Safe Retrying

---

## 1. Introduction (Why?)

When working with databases, errors **will happen**:

* Connection failures (network issues, DB down)
* Query syntax errors
* Timeouts or deadlocks
* Constraint violations (e.g., unique key conflicts)

**Proper error handling** ensures:

* Application stability
* Safe retries for transient failures
* Clear logging and debugging

Tokio Postgres exposes **`tokio_postgres::Error`**, which can be inspected for different kinds of failures.

---

## 2. Syntax / Core Idea

```rust
match client.execute("INVALID QUERY", &[]).await {
    Ok(rows) => println!("Success, affected rows: {}", rows),
    Err(e) => eprintln!("Database error: {}", e),
}
```

* Use `Result<T, tokio_postgres::Error>` for **error handling**.
* `?` can propagate errors upward.

---

## 3. Full Real-World Examples

---

### Example 1: Basic Error Handling

```rust
let result = client.execute("UPDATE users SET name = $1 WHERE id = $2", &[&"Alice", &1]).await;

match result {
    Ok(rows) => println!("Updated {} rows", rows),
    Err(e) => eprintln!("Error updating user: {}", e),
}
```

* Simple pattern → **log and continue**.

---

### Example 2: Using `?` for Propagation

```rust
async fn update_user_name(client: &tokio_postgres::Client, id: i32, name: &str) -> Result<u64, tokio_postgres::Error> {
    let rows = client.execute("UPDATE users SET name = $1 WHERE id = $2", &[&name, &id]).await?;
    Ok(rows)
}
```

* Propagates error to caller → cleaner **function design**.

---

### Example 3: Retry on Transient Errors

```rust
use tokio::time::{sleep, Duration};

let mut retries = 3;
while retries > 0 {
    match client.execute("UPDATE accounts SET balance = balance - 100 WHERE id=$1", &[&1]).await {
        Ok(rows) => {
            println!("Updated {} rows", rows);
            break;
        }
        Err(e) => {
            eprintln!("Error: {}. Retrying...", e);
            retries -= 1;
            sleep(Duration::from_secs(1)).await;
        }
    }
}
```

* Retries on temporary failures (network issues, deadlocks).
* Adds **delay** between retries → avoid overwhelming DB.

---

### Example 4: Handling Connection Errors

```rust
let conn_result = tokio_postgres::connect("host=localhost user=postgres password=wrong dbname=mydb", tokio_postgres::NoTls).await;

match conn_result {
    Ok((client, connection)) => println!("Connected successfully!"),
    Err(e) => eprintln!("Failed to connect: {}", e),
}
```

* Useful for **startup checks** or reconnect logic.

---

### Example 5: Handling Query Constraint Violations

```rust
let result = client.execute("INSERT INTO users (email) VALUES ($1)", &[&"duplicate@example.com"]).await;

match result {
    Ok(_) => println!("Inserted successfully"),
    Err(e) => {
        if let Some(db_error) = e.as_db_error() {
            println!("DB error code: {}, message: {}", db_error.code(), db_error.message());
        } else {
            println!("Other error: {}", e);
        }
    }
}
```

* Allows **specific handling for constraint violations** or custom logic.

---

### Example 6: Combining Retry with Transactions

```rust
use tokio::time::sleep;

let mut retries = 3;
while retries > 0 {
    let txn = client.transaction().await?;
    let result: Result<(), tokio_postgres::Error> = async {
        txn.execute("UPDATE accounts SET balance = balance - 100 WHERE id=$1", &[&1]).await?;
        txn.execute("UPDATE accounts SET balance = balance + 100 WHERE id=$2", &[&2]).await?;
        txn.commit().await?;
        Ok(())
    }.await;

    match result {
        Ok(_) => {
            println!("Transaction successful");
            break;
        }
        Err(e) => {
            eprintln!("Transaction failed: {}. Retrying...", e);
            retries -= 1;
            sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
```

* Handles **transient failures in multi-step operations** safely.

---

## 4. Best Practices & Tips

* ✅ Always check errors when calling `execute`, `query`, or `transaction`.
* ✅ Use `as_db_error()` to **inspect Postgres-specific error codes**.
* ✅ Retry **only for transient errors**, not permanent ones (like syntax mistakes).
* ✅ Log errors with sufficient context → easier debugging.
* ✅ Combine retries with **backoff strategies** in production for reliability.

---

## 5. Assignments (Practice)

1. Attempt a query with an invalid SQL and handle the error gracefully.
2. Implement a retry mechanism for a query that may fail due to transient network issues.
3. Connect with wrong credentials and handle the connection error.
4. Handle a constraint violation (e.g., inserting duplicate keys).
5. Combine retry logic with a transaction that updates multiple rows.

---

✅ That completes **Page 9: Handling Common Errors and Safe Retrying**.

Do you want me to continue with **Page 10: Advanced Techniques – Batch Operations & Transactions Optimization** next?
