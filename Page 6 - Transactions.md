Content for Page 6: Transactions
Absolutely! 🚀 Let’s move on to **Page 6: Transactions** in the Tokio Postgres guide. This is a crucial topic because it allows trainees to handle **multiple operations safely and atomically**.

---

# 📘 Page 6: Transactions

---

## 1. Introduction (Why?)

In real-world applications, many operations **must succeed together** or fail together.

* Example: Transferring money from one account to another:

  1. Deduct from account A
  2. Add to account B

If only step 1 succeeds and step 2 fails → data inconsistency.

👉 **Transactions** ensure atomicity:

* **Commit** → all operations succeed
* **Rollback** → all operations fail if something goes wrong

Tokio Postgres allows **async transactions** using `client.transaction().await`.

---

## 2. Syntax / Core Idea

```rust
let transaction = client.transaction().await?; // start transaction

// Run queries inside transaction
transaction.execute("UPDATE accounts SET balance = balance - 100 WHERE id = $1", &[&1]).await?;
transaction.execute("UPDATE accounts SET balance = balance + 100 WHERE id = $2", &[&2]).await?;

// Commit transaction
transaction.commit().await?;
```

* If any query fails before `commit`, **rollback happens automatically** (or manually).

---

## 3. Full Real-World Examples

---

### Example 1: Basic Transaction

```rust
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=1234 dbname=mydb", NoTls
    ).await?;

    tokio::spawn(async move { let _ = connection.await; });

    // Start a transaction
    let txn = client.transaction().await?;

    // Deduct from Alice
    txn.execute(
        "UPDATE accounts SET balance = balance - 50 WHERE id = $1",
        &[&1]
    ).await?;

    // Add to Bob
    txn.execute(
        "UPDATE accounts SET balance = balance + 50 WHERE id = $2",
        &[&2]
    ).await?;

    // Commit
    txn.commit().await?;
    println!("Transaction committed successfully!");
    Ok(())
}
```

✅ Ensures **all operations succeed or fail together**.

---

### Example 2: Transaction with Rollback on Error

```rust
let txn = client.transaction().await?;

if let Err(e) = async {
    txn.execute("UPDATE accounts SET balance = balance - 100 WHERE id=$1", &[&1]).await?;
    txn.execute("UPDATE accounts SET balance = balance + 100 WHERE id=$2", &[&2]).await?;
    Ok::<_, tokio_postgres::Error>(())
}.await {
    txn.rollback().await?;
    eprintln!("Transaction failed and rolled back: {}", e);
}
```

* Demonstrates **manual rollback** when a query fails.

---

### Example 3: Nested Transactions (Savepoints)

```rust
let txn = client.transaction().await?;

// Create a savepoint
let savepoint = txn.savepoint("sp1").await?;

// Try some operation
if let Err(e) = savepoint.execute("UPDATE accounts SET balance = balance - 500 WHERE id=$1", &[&1]).await {
    savepoint.rollback().await?;
    println!("Rolled back to savepoint due to error: {}", e);
}

// Continue other operations
txn.execute("UPDATE accounts SET balance = balance + 500 WHERE id=$2", &[&2]).await?;
txn.commit().await?;
```

* Nested transactions are useful when **partial rollback** is needed inside a bigger transaction.

---

### Example 4: Using `RETURNING` Inside Transaction

```rust
let txn = client.transaction().await?;

let row = txn.query_one(
    "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
    &[&"Charlie", &"charlie@example.com"]
).await?;

let new_user_id: i32 = row.get("id");
println!("Inserted user ID: {}", new_user_id);

txn.commit().await?;
```

* Can return inserted or updated data **without leaving the transaction**.

---

### Example 5: Multiple Inserts in One Transaction

```rust
let txn = client.transaction().await?;
let users = vec![
    ("David", "david@example.com"),
    ("Eve", "eve@example.com"),
];

for (name, email) in users {
    txn.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&name, &email]
    ).await?;
}

txn.commit().await?;
println!("Inserted multiple users atomically!");
```

* Demonstrates **batch operations inside a transaction**.

---

## 4. Best Practices & Tips

* ✅ Always **commit** or **rollback** transactions.
* ✅ Use transactions for **related operations** only. Don’t wrap unrelated queries unnecessarily.
* ✅ Prefer **short-lived transactions** → reduces locks and improves concurrency.
* ✅ Use **savepoints** for nested operations that might fail individually.
* ❌ Never keep a transaction open for long periods (blocks other queries).
* ✅ Always handle errors with `?` or manual rollback.

---

## 5. Assignments (Practice)

1. Create a transaction that transfers money between two accounts and commits.
2. Modify it to rollback if the sender has insufficient balance.
3. Use `RETURNING` inside a transaction to get inserted row IDs.
4. Create nested transactions with savepoints and simulate an error.
5. Insert multiple rows inside a single transaction and verify atomicity.

---

✅ That completes **Page 6 (Transactions)**.

Do you want me to continue with **Page 7: Prepared Statements and Reuse** next?
