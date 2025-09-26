Content for Page 8: Mapping Rows to Rust Structs
# 📘 Page 8: Mapping Rows to Rust Structs

---

## 1. Introduction (Why?)

When querying PostgreSQL, results are returned as `Row` objects.

* Using raw `Row` objects everywhere is **tedious and error-prone**.
* Mapping rows to **Rust structs** makes the code **type-safe, readable, and maintainable**.

Two main approaches:

1. **Manual mapping** → extract values from `Row` and assign to struct fields.
2. **Trait-based / custom FromRow pattern** → reusable conversion logic.

This page focuses on **manual mapping**, which is essential before using higher-level ORMs.

---

## 2. Syntax / Core Idea

```rust
struct User {
    id: i32,
    name: String,
    email: String,
}

// Manual mapping from a Row
let row: Row = client.query_one("SELECT id, name, email FROM users WHERE id=$1", &[&1]).await?;
let user = User {
    id: row.get("id"),
    name: row.get("name"),
    email: row.get("email"),
};
```

* `row.get("column_name")` → extracts a typed value.
* Can also use `row.get(index)` → by column index.

---

## 3. Full Real-World Examples

---

### Example 1: Simple Struct Mapping

```rust
use tokio_postgres::{NoTls, Row};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=mydb", NoTls).await?;
    tokio::spawn(async move { let _ = connection.await; });

    let row: Row = client.query_one("SELECT id, name, email FROM users WHERE id=$1", &[&1]).await?;
    let user = User {
        id: row.get("id"),
        name: row.get("name"),
        email: row.get("email"),
    };

    println!("Fetched user: {:?}", user);
    Ok(())
}
```

* Simple and direct mapping → ideal for small queries.

---

### Example 2: Mapping Multiple Rows

```rust
let rows = client.query("SELECT id, name, email FROM users", &[]).await?;
let users: Vec<User> = rows.iter().map(|row| User {
    id: row.get("id"),
    name: row.get("name"),
    email: row.get("email"),
}).collect();

for user in users {
    println!("User: {:?}", user);
}
```

* Converts **multiple rows into a vector of structs**.

---

### Example 3: Mapping Optional Columns

```rust
struct Profile {
    id: i32,
    bio: Option<String>, // may be NULL
}

let row = client.query_one("SELECT id, bio FROM profiles WHERE id=$1", &[&1]).await?;
let profile = Profile {
    id: row.get("id"),
    bio: row.get("bio"), // Option<T> handles NULL safely
};

println!("Profile: {:?}", profile);
```

* `Option<T>` allows mapping **nullable columns safely**.

---

### Example 4: Nested Structs

```rust
struct Account {
    id: i32,
    balance: f64,
    owner: User,
}

let row = client.query_one("SELECT a.id, a.balance, u.id, u.name, u.email FROM accounts a JOIN users u ON a.owner_id = u.id WHERE a.id=$1", &[&1]).await?;

let account = Account {
    id: row.get("id"),
    balance: row.get("balance"),
    owner: User {
        id: row.get(2),         // column index for joined table
        name: row.get(3),
        email: row.get(4),
    },
};

println!("Account: {:?}", account);
```

* Handles **joined queries** with nested Rust structs.

---

### Example 5: Using a Helper Function

```rust
fn row_to_user(row: &Row) -> User {
    User {
        id: row.get("id"),
        name: row.get("name"),
        email: row.get("email"),
    }
}

let rows = client.query("SELECT id, name, email FROM users", &[]).await?;
let users: Vec<User> = rows.iter().map(row_to_user).collect();
```

* Encapsulates mapping logic → **reduces repeated code**.

---

### Example 6: Mapping with Indexes

```rust
let row = client.query_one("SELECT id, name, email FROM users WHERE id=$1", &[&1]).await?;
let user = User {
    id: row.get(0),    // first column
    name: row.get(1),
    email: row.get(2),
};
```

* Faster than column names if **order is known**, but less readable.

---

## 4. Best Practices & Tips

* ✅ Prefer **column names** for readability; indexes for performance when necessary.
* ✅ Use **Option<T>** for nullable columns.
* ✅ Wrap mapping logic in helper functions for **reusability**.
* ✅ Always match types correctly (`i32`, `String`, `f64`) → avoid runtime panics.
* ❌ Avoid manually parsing values from `row.get::<_, String>(index)` for unrelated types.

---

## 5. Assignments (Practice)

1. Map a single user row into a Rust struct.
2. Map all users in a table into a vector of structs.
3. Handle nullable columns using `Option<T>`.
4. Map results of a JOIN query into nested structs.
5. Create a helper function for row → struct mapping and reuse it for multiple queries.

---

✅ That completes **Page 8: Mapping Rows to Rust Structs**.

Do you want me to continue with **Page 9: Handling Common Errors and Safe Retrying** next?
