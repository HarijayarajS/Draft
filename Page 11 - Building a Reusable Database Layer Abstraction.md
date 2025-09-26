Content for Page 11: Building a Reusable Database Layer Abstraction

Absolutely! Let’s continue with **Page 12: Best Practices & Advanced Topics**. This page focuses on **guidelines and strategies** to write **robust, high-performance, and maintainable async Rust applications with Tokio Postgres**.

---

# 📘 Page 12: Best Practices & Advanced Topics

---

## 1. Introduction (Why?)

Once you know the basics of Tokio Postgres, it’s crucial to:

* Ensure **performance** and scalability
* Avoid **common pitfalls** in async database applications
* Write **secure, maintainable, and testable** code
* Handle **complex real-world scenarios**

This page summarizes **advanced strategies and best practices** for production-ready applications.

---

## 2. Syntax / Core Idea

There is no single code snippet here — it’s about **patterns and conventions**:

* Use **connection pooling** (`deadpool-postgres`)
* Always **prepare statements** for repeated queries
* Wrap multi-step operations in **transactions**
* Handle errors using **`tokio_postgres::Error`** and **retries**
* Keep **transactions short-lived** to avoid locks

---

## 3. Full Real-World Examples

---

### Example 1: Using Connection Pool Effectively

```rust
use deadpool_postgres::{Pool, Client};

async fn get_user_email(pool: &Pool, user_id: i32) -> Result<String, tokio_postgres::Error> {
    // Get a pooled client
    let client: Client = pool.get().await.map_err(|e| tokio_postgres::Error::from(e))?;
    
    // Query the database
    let row = client.query_one("SELECT email FROM users WHERE id=$1", &[&user_id]).await?;
    
    Ok(row.get("email"))
}
```

* Use **pool.get()** instead of creating new connections each time.
* Improves **concurrency and performance** in async apps.

---

### Example 2: Short-Lived Transactions

```rust
let txn = client.transaction().await?;
txn.execute("UPDATE accounts SET balance = balance - 100 WHERE id=$1", &[&1]).await?;
txn.execute("UPDATE accounts SET balance = balance + 100 WHERE id=$2", &[&2]).await?;
txn.commit().await?;
```

* Keep transactions **small and atomic** to prevent locks.
* Avoid long-running operations inside transactions.

---

### Example 3: Parameterized Queries (Security)

```rust
let username = "alice";
let row = client.query_one("SELECT * FROM users WHERE username=$1", &[&username]).await?;
```

* **Always use parameters** (`$1`, `$2`) instead of string interpolation
* Prevents **SQL injection attacks**

---

### Example 4: Prepared Statements for Performance

```rust
let stmt = client.prepare("INSERT INTO logs (message) VALUES ($1)").await?;
client.execute(&stmt, &[&"User logged in"]).await?;
```

* Prepare once → reuse for multiple inserts
* Reduces **parsing overhead** and improves throughput

---

### Example 5: Efficient Error Handling

```rust
match client.execute("UPDATE accounts SET balance = balance - $1 WHERE id=$2", &[&100, &1]).await {
    Ok(_) => println!("Success"),
    Err(e) => eprintln!("Database error: {:?}", e),
}
```

* Log errors with **enough context**
* Retry **only transient failures** like network timeouts

---

### Example 6: Struct Mapping & Abstraction

```rust
struct User {
    id: i32,
    name: String,
    email: String,
}

let row = client.query_one("SELECT id, name, email FROM users WHERE id=$1", &[&1]).await?;
let user = User {
    id: row.get("id"),
    name: row.get("name"),
    email: row.get("email"),
};
```

* Map rows to **Rust structs** → easier to work with in code
* Combine with a **DB abstraction layer** for maintainability

---

### Example 7: Combining Retry + Backoff

```rust
use tokio::time::{sleep, Duration};

let mut retries = 5;
while retries > 0 {
    match client.execute("UPDATE accounts SET balance = balance - $1 WHERE id=$2", &[&100, &1]).await {
        Ok(_) => break,
        Err(e) => {
            eprintln!("Error: {}. Retrying...", e);
            retries -= 1;
            sleep(Duration::from_secs(2)).await; // simple backoff
        }
    }
}
```

* Prevents overwhelming DB on transient failures
* Implement **exponential backoff** for production

---

## 4. Best Practices & Tips

* ✅ **Connection Management**: Use pooled connections for async apps.
* ✅ **Transactions**: Keep them short and atomic; use savepoints if needed.
* ✅ **Prepared Statements**: Always prepare queries reused multiple times.
* ✅ **Error Handling**: Inspect `tokio_postgres::Error` and handle retries carefully.
* ✅ **Security**: Use parameterized queries to avoid SQL injection.
* ✅ **Struct Mapping**: Map DB rows to Rust structs, centralize in DB layer.
* ✅ **Logging & Monitoring**: Always log queries, errors, and execution times.
* ✅ **Testing**: Use mock databases or test containers to simulate DB operations.
* ✅ **Performance**: Batch inserts/updates and avoid long-running queries.
* ✅ **Async Best Practices**: Don’t block async tasks; offload heavy computation if needed.

---

## 5. Assignments (Practice)

1. Implement a pooled connection method and fetch multiple rows asynchronously.
2. Use parameterized queries to fetch users by different filters.
3. Write a small transaction with rollback on failure.
4. Add prepared statements for repeated insert/update operations.
5. Implement retry logic with backoff for transient errors.
6. Map query results into Rust structs and use them in functions.
7. Log all query execution times and errors for monitoring purposes.

---

✅ That completes **Page 12: Best Practices & Advanced Topics**.

This concludes the **full guide from basics to advanced** for **Tokio Postgres in Rust**.

If you want, I can **compile all 12 pages into a full Reveal.js slide deck** with detailed explanations and code examples for each page.

Do you want me to do that next?
