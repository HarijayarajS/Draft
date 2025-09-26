Content for Page 10: Batch Operations and Transactions Optimization

Absolutely! Let’s continue with **Page 10: Advanced Techniques – Batch Operations & Transactions Optimization**. This page is essential for teaching trainees how to **handle large-scale database operations efficiently and safely**.

---

# 📘 Page 10: Advanced Techniques – Batch Operations & Transactions Optimization

---

## 1. Introduction (Why?)

In production systems, you often need to:

* Insert, update, or delete **many rows at once**
* Avoid **performance bottlenecks** due to repeated single queries
* Ensure **atomicity** when multiple operations are logically connected

**Batch operations** + **optimized transactions**:

* Reduce network round-trips
* Leverage prepared statements
* Maintain consistency in case of failures

---

## 2. Syntax / Core Idea

**Batch Insert Using `execute` in Loops or Transactions:**

```rust
let txn = client.transaction().await?;
for user in users {
    txn.execute("INSERT INTO users (name, email) VALUES ($1, $2)", &[&user.name, &user.email]).await?;
}
txn.commit().await?;
```

**Optimizations:**

* Prepare the query once → reuse
* Use transactions for **all inserts together**
* Consider bulk inserts (`COPY` or `UNNEST`) for very large datasets

---

## 3. Full Real-World Examples

---

### Example 1: Batch Insert with Transaction

```rust
let users = vec![
    ("Alice", "alice@example.com"),
    ("Bob", "bob@example.com"),
    ("Charlie", "charlie@example.com"),
];

let txn = client.transaction().await?;
let stmt = txn.prepare("INSERT INTO users (name, email) VALUES ($1, $2)").await?;

for (name, email) in users {
    txn.execute(&stmt, &[&name, &email]).await?;
}

txn.commit().await?;
println!("Batch insert completed successfully!");
```

* Combines **prepared statements + transaction** for performance and safety.

---

### Example 2: Batch Update

```rust
let updates = vec![(1, 1200.0), (2, 800.0), (3, 1500.0)]; // (id, new_balance)
let txn = client.transaction().await?;
let stmt = txn.prepare("UPDATE accounts SET balance = $1 WHERE id = $2").await?;

for (id, balance) in updates {
    txn.execute(&stmt, &[&balance, &id]).await?;
}

txn.commit().await?;
println!("Batch update completed!");
```

* Efficiently updates multiple rows in a **single transaction**.

---

### Example 3: Batch Delete

```rust
let user_ids = vec![4, 5, 6];
let txn = client.transaction().await?;
let stmt = txn.prepare("DELETE FROM users WHERE id = $1").await?;

for id in user_ids {
    txn.execute(&stmt, &[&id]).await?;
}

txn.commit().await?;
println!("Batch delete executed safely!");
```

* Ensures **all deletes succeed together** or none are applied.

---

### Example 4: Using `UNNEST` for Large Batch Inserts

```rust
let names = vec!["David", "Eve"];
let emails = vec!["david@example.com", "eve@example.com"];

client.execute(
    "INSERT INTO users (name, email) SELECT * FROM UNNEST($1::text[], $2::text[])",
    &[&names, &emails]
).await?;
println!("Large batch insert using UNNEST completed!");
```

* Avoids multiple network round-trips → ideal for **thousands of rows**.

---

### Example 5: Optimized Transaction with Error Handling

```rust
let txn = client.transaction().await?;
let stmt = txn.prepare("INSERT INTO logs (message) VALUES ($1)").await?;

let messages = vec!["Start process", "Processing step 1", "Processing step 2"];

if let Err(e) = async {
    for msg in messages {
        txn.execute(&stmt, &[&msg]).await?;
    }
    Ok::<(), tokio_postgres::Error>(())
}.await {
    txn.rollback().await?;
    eprintln!("Transaction failed and rolled back: {}", e);
} else {
    txn.commit().await?;
    println!("Transaction committed successfully!");
}
```

* Combines **batch operations, transactions, and error handling** for robust code.

---

### Example 6: Nested Batch Transactions (Savepoints)

```rust
let txn = client.transaction().await?;
let savepoint = txn.savepoint("sp1").await?;

if let Err(e) = savepoint.execute("INSERT INTO accounts (id, balance) VALUES ($1, $2)", &[&10, &500.0]).await {
    savepoint.rollback().await?;
    println!("Rolled back savepoint due to error: {}", e);
}

txn.commit().await?;
```

* Allows **partial rollback** within larger batch operations → fine-grained control.

---

## 4. Best Practices & Tips

* ✅ Always wrap batch operations in **transactions**.
* ✅ Prepare queries **once** for multiple executions.
* ✅ Use **savepoints** for nested operations with possible failures.
* ✅ Consider `UNNEST` or `COPY` for **very large datasets**.
* ✅ Keep transactions **short-lived** → avoid locks and contention.
* ✅ Handle errors explicitly → rollback entire batch if needed.

---

## 5. Assignments (Practice)

1. Insert 10+ users using a single transaction and prepared statement.
2. Update multiple accounts balances in a batch safely.
3. Delete a batch of users inside a transaction.
4. Use `UNNEST` to insert multiple rows efficiently.
5. Combine nested transactions with savepoints and simulate an error to rollback partially.

---

✅ That completes **Page 10: Advanced Techniques – Batch Operations & Transactions Optimization**.

Do you want me to continue with **Page 11: Building a Reusable Database Layer / Abstraction** next?
