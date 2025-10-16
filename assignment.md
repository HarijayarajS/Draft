Of course. Here is a Level-2 assignment that introduces data relationships.

### Title

Blog API – Managing Posts and Comments

### Description

  - The purpose of this assignment is to build a web API for a simple blog. You will manage two main data types: `Post` and `Comment`, creating a **one-to-many relationship** where one post can have many comments.
  - Your server will handle creating and managing blog posts. It will also allow users to add comments to a specific post and list all comments for that post. This will teach you how to handle related data in an API and database.
  - The data flow and relationship look like this:

<!-- end list -->

```rust
// Client --> Axum Server --> PostgreSQL Database (with two related tables)
//
// [Client] --POST /posts/1/comments--> [Axum Router] --> [Comment Handler] --INSERT INTO comments ...--> [DB]
//                                                                                                         ^
//                                                                                                         |
//  +-----------+                                                                                   +--------+-------+
//  | posts     | 1                                                                                 | comments       |
//  +-----------+                                                                                   +----------------+
//  | id (PK)   | <-------------------------------------------------------------------------------* | id (PK)        |
//  | title     |                                                                                   | post_id (FK)   |
//  | body      |                                                                                   | content        |
//  +-----------+                                                                                   +----------------+
```

### Folder & File Structure

```
blog_api/
└── src/
    └── main.rs
```

### Implementation details

  - **Main Types:** `Post` and `Comment`.

  - **Internal Structure:**

      - `Post`: A struct with `id` (integer), `title` (string), `body` (string), and `published_at` (`chrono::NaiveDateTime`).
      - `Comment`: A struct with `id` (integer), `post_id` (integer, a **foreign key** to `Post`), `author` (string), and `content` (string).
      - `CreatePost`: A helper struct for creating a post (without the `id` or `published_at` fields).
      - `CreateComment`: A helper struct for creating a comment (without `id` or `post_id`, as the ID comes from the path).

  - **Functions to Implement:**

      - `async fn create_post(State(pool): State<PgPool>, Json(payload): Json<CreatePost>) -> Result<Json<Post>, ...>`
          - **Explanation:** Creates a new blog post in the `posts` table. The `published_at` field should be set to the current UTC time upon creation.
      - `async fn list_posts(State(pool): State<PgPool>) -> Result<Json<Vec<Post>>, ...>`
          - **Explanation:** Fetches and returns all posts from the database.
      - `async fn get_post(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Post>, ...>`
          - **Explanation:** Fetches a single blog post by its unique `id`.
      - `async fn create_comment_for_post(State(pool): State<PgPool>, Path(post_id): Path<i32>, Json(payload): Json<CreateComment>) -> Result<Json<Comment>, ...>`
          - **Explanation:** This is a key function. It extracts the `post_id` from the URL path. It then creates a new comment in the `comments` table, linking it to the correct post using the `post_id`.
      - `async fn list_comments_for_post(State(pool): State<PgPool>, Path(post_id): Path<i32>) -> Result<Json<Vec<Comment>>, ...>`
          - **Explanation:** Fetches and returns all comments that belong to a specific `post_id` provided in the URL path.
      - `#[tokio::main] async fn main()`
          - **Explanation:** The main function that sets up the database connection pool, defines the API routes (including the **nested route** for comments), and starts the server.

-----

### Mini Glossary

  - **Foreign Key (FK)**: A key used to link two tables together. The `post_id` column in the `comments` table is a foreign key because it refers to the `id` (primary key) in the `posts` table, creating a relationship.
  - **Path Extractor**: An Axum feature that deserializes parameters from the URL's path. In this assignment, you'll use `Path(i32)` and `Path<(i32, i32)>` to capture IDs like `/posts/:id` and `/posts/:post_id/comments`.
  - **Error Handling (`Result<T, E>`)**: Rust's primary mechanism for handling operations that might fail. When dealing with databases and user input, operations can fail (e.g., post not found). `Result` forces you to handle both the success (`Ok`) and failure (`Err`) cases.
  - **Ownership**: A core Rust concept where every value has a single "owner." This is crucial for managing memory safely without a garbage collector, especially in a concurrent environment like a web server where data is passed between functions.

-----

### Timing & Accept criteria

  - **Time:** \~1 hour.

  - **Acceptance Checklist:**

    1.  Create the `posts` and `comments` tables in your PostgreSQL database with the correct primary and foreign key relationship.
    2.  Define the `Post`, `Comment`, `CreatePost`, and `CreateComment` structs in Rust.
    3.  Implement all handlers for post CRUD operations.
    4.  Implement the two handlers for creating and listing comments, correctly using the `post_id` from the URL path.
    5.  Configure the Axum `Router` with nested routes for `/posts/:post_id/comments`.
    6.  Demonstrate using `curl` that you can create a post, and then add and list comments for that specific post.

  - **Expected Output Snippets:**

      - Creating a post:
        ```sh
        $ curl -X POST -H "Content-Type: application/json" -d '{"title": "My First Post", "body": "..."}' http://127.0.0.1:3000/posts
        {"id":1,"title":"My First Post","body":"...","published_at":"2025-10-16T07:00:00"}
        ```
      - Adding a comment to that post:
        ```sh
        $ curl -X POST -H "Content-Type: application/json" -d '{"author": "Alice", "content": "Great article!"}' http://127.0.0.1:3000/posts/1/comments
        {"id":1,"post_id":1,"author":"Alice","content":"Great article!"}
        ```
      - Listing comments for that post:
        ```sh
        $ curl http://127.0.0.1:3000/posts/1/comments
        [{"id":1,"post_id":1,"author":"Alice","content":"Great article!"}]
        ```

-----

### Demo

This starter code shows how to structure the nested routes. You need to implement the structs and the logic within the handlers.

```rust
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;
use tokio;

// TODO: Define your Post, Comment, CreatePost, and CreateComment structs here.

// TODO: Implement handlers for post CRUD (create_post, list_posts, get_post).
async fn list_posts() -> &'static str { "List of all posts" }

// TODO: Implement handlers for comments (create_comment_for_post, list_comments_for_post).
async fn list_comments_for_post(Path(post_id): Path<i32>) -> String {
    format!("List of comments for post {}", post_id)
}

#[tokio::main]
async fn main() {
    // TODO: Set up your database connection pool.

    let app = Router::new()
        .route("/posts", get(list_posts).post(/* create_post */))
        .route("/posts/:id", get(/* get_post */))
        .route("/posts/:post_id/comments", get(list_comments_for_post).post(/* create_comment_for_post */));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("📝 Blog API server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Expected output when the server starts:
// 📝 Blog API server listening on 127.0.0.1:3000
```


Of course\! Here is the updated assignment for the Personal Finance Tracker, now including an endpoint for filtered reports.

-----

### Title

Personal Finance Tracker – CRUD & Reports API

### Description

  - The goal is to build a web API to track personal financial transactions. The main type will be a `Transaction` struct, which now includes a date.
  - Your server will handle requests to create, read, update, and delete transactions. Additionally, you will build a powerful **report endpoint** that can filter transactions by type and date range to provide a financial summary. All data is stored in a PostgreSQL database.
  - The data flow for a report request looks like this:

<!-- end list -->

```rust
// Client (e.g., curl) --GET with query params--> Axum Server --> PostgreSQL Database
//
// [Client] --/report?type=Income--> [Axum Router] --> [Report Handler] --SELECT ... WHERE type='Income'--> [Postgres]
//   ^                                                                                                        |
//   |---------------------------------------JSON Report Response---------------------------------------------|
```

### Folder & File Structure

```
finance_tracker/
└── src/
    └── main.rs
```

### Implementation details

  - **Main Type:** `Transaction`

  - **Internal Structure:**

      - `TransactionType`: An `enum` with two variants, `Income` and `Expense`.
      - `Transaction`: A struct containing `id` (integer), `description` (string), `amount` (`f64`), `transaction_type` (`TransactionType`), and `transaction_date` (`chrono::NaiveDate`).
      - `CreateTransaction`: A helper struct like `Transaction` but without the `id`.
      - **`ReportFilters`**: A struct to capture optional query parameters from the URL (`transaction_type: Option<String>`, `start_date: Option<NaiveDate>`, `end_date: Option<NaiveDate>`).
      - **`TransactionReport`**: A struct for the report response, containing fields like `total_income`, `total_expenses`, `net_balance`, `count`, and a `Vec<Transaction>` of the filtered results.

  - **Functions to Implement:**

      - `async fn create_transaction(...) -> Result<Json<Transaction>, ...>`
          - **Explanation:** Saves a new transaction (including its date) to the database.
      - `async fn list_transactions(...) -> Result<Json<Vec<Transaction>>, ...>`
          - **Explanation:** Fetches and returns all transactions from the database.
      - `async fn get_transaction(...) -> Result<Json<Transaction>, ...>`
          - **Explanation:** Fetches a single transaction by its `id`.
      - `async fn update_transaction(...) -> Result<Json<Transaction>, ...>`
          - **Explanation:** Updates an existing transaction's details in the database.
      - `async fn delete_transaction(...) -> StatusCode`
          - **Explanation:** Removes a transaction from the database by its `id`.
      - `async fn get_transaction_report(State(pool): State<PgPool>, Query(filters): Query<ReportFilters>) -> Result<Json<TransactionReport>, ...>`
          - **Explanation:** This is the new report function. It takes optional filters from the URL's query string (e.g., `?type=Income`). It then builds a dynamic SQL query to fetch only the matching transactions, calculates the total income/expenses for that set, and returns a complete summary report as JSON.
      - `#[tokio::main] async fn main()`
          - **Explanation:** Sets up the database connection pool, defines all API routes including the new `/transactions/report` route, and starts the Axum web server.

-----

### Mini Glossary

  - **`enum`**: A type that can be one of several possible variants. We use it for `TransactionType` to ensure data integrity, so a transaction must be either `Income` or `Expense`.
  - **Derive Macros**: Attributes like `#[derive(Serialize, Deserialize)]` that automatically generate standard trait implementations for your structs, saving a lot of boilerplate code.
  - **`State` Extractor**: An Axum feature that allows you to share data, like a database connection pool, across all your request handlers safely and efficiently.
  - **`Query` Extractor**: An Axum feature used to deserialize data from the URL's query string (the part after `?`). We use `Query<ReportFilters>` to easily access the optional filters for our report handler.

-----

### Timing & Accept criteria

  - **Time:** \~1 hour.

  - **Acceptance Checklist:**

    1.  Update the `Transaction` and `CreateTransaction` structs to include a `transaction_date` field.
    2.  Define the `ReportFilters` and `TransactionReport` structs.
    3.  Implement all five CRUD handlers to correctly manage the date field.
    4.  Implement the `get_transaction_report` handler, which dynamically builds a SQL query based on the filters.
    5.  Add the new `/transactions/report` route to the Axum `Router`.
    6.  Demonstrate using `curl` that all endpoints work, paying special attention to the report endpoint with different filter combinations.

  - **Expected Output Snippets:**

      - Creating a transaction with a date:
        ```sh
        $ curl -X POST -H "Content-Type: application/json" -d '{"description": "Groceries", "amount": 75.50, "transaction_type": "Expense", "transaction_date": "2025-10-16"}' http://127.0.0.1:3000/transactions
        {"id":1,"description":"Groceries","amount":75.5,"transaction_type":"Expense","transaction_date":"2025-10-16"}
        ```
      - Getting a filtered report for all expenses:
        ```sh
        $ curl "http://127.0.0.1:3000/transactions/report?transaction_type=Expense"
        {"total_income":0.0,"total_expenses":75.5,"net_balance":-75.5,"count":1,"transactions":[{"id":1,"description":"Groceries","amount":75.5,"transaction_type":"Expense","transaction_date":"2025-10-16"}]}
        ```

-----

### Demo

This starter code includes the new report route. Your task is to define the structs and implement the logic for all handlers.

```rust
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use chrono::NaiveDate;

// TODO: Define your Transaction, CreateTransaction, TransactionType,
// ReportFilters, and TransactionReport structs here.

// TODO: Implement the five CRUD handler functions.

// Placeholder for the new report handler
async fn get_transaction_report() -> &'static str {
    // TODO: Implement filtering and aggregation logic here.
    "This will be a JSON report summary."
}


#[tokio::main]
async fn main() {
    // TODO: Set up your database connection pool.

    let app = Router::new()
        .route("/transactions", get(/* list */).post(/* create */))
        .route("/transactions/report", get(get_transaction_report))
        .route("/transactions/:id", get(/* get */).put(/* update */).delete(/* delete */));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Expected output when the server starts:
// 🚀 Server listening on 127.0.0.1:3000
```


Of course. Here is a Level-2 assignment that shifts from web APIs to building a command-line tool, a common use case for Rust.

### Title

Simple Migrator – A CLI for PostgreSQL Schema Migrations

### Description

  - The purpose of this assignment is to build a command-line interface (CLI) tool that manages database schema changes. The main logic will be encapsulated in a `Migrator` struct.
  - Your tool will read `.sql` files from a dedicated `migrations/` folder. It will keep track of which migrations have already been applied by using a special table in the database (e.g., `schema_migrations`). When run, it will execute only the new, unapplied SQL files in chronological order.
  - The workflow is as follows:

<!-- end list -->

```rust
// User runs the tool from the terminal
//
// [cargo run -- migrate] --> [CLI App] --> [Reads ./migrations/*.sql files]
//                                |
//                                +--> [Connects to PostgreSQL]
//                                |
//                                +--> [Creates `schema_migrations` table if not exists]
//                                |
//                                +--> [Compares files with records in table]
//                                |
//                                +--> [Executes new .sql files in a transaction]
//                                |
//                                +--> [Records newly run files in `schema_migrations` table]
```

### Folder & File Structure

```
db_migrator/
├── migrations/
│   ├── 001_create_posts_table.sql
│   └── 002_create_comments_table.sql
└── src/
    └── main.rs
```

### Implementation details

  - **Main Type:** `Migrator`

  - **Internal Structure:**

      - `Cli`: A struct defined using the `clap` crate to parse command-line arguments. It should define a `migrate` subcommand.
      - `Migrator`: A struct that holds the database connection client (`tokio_postgres::Client`).
      - SQL files in the `migrations/` directory should be named with a leading number to ensure correct execution order (e.g., `001_...`, `002_...`).

  - **Functions/Methods to Implement:**

      - `#[tokio::main] async fn main() -> anyhow::Result<()>`
          - **Explanation:** The program's entry point. It will parse command-line arguments using `clap`, establish a connection to the PostgreSQL database, create an instance of `Migrator`, and call its main `run` method.
      - `impl Migrator { ... }`
          - `async fn run(&mut self) -> anyhow::Result<()>`: The core logic. It ensures the migrations table exists, gets the list of already applied migrations, reads all `.sql` files from the filesystem, determines which new files to run, and applies them.
          - `async fn ensure_migrations_table(&mut self) -> anyhow::Result<()>`: Checks if a `schema_migrations` table (with a single `version` column) exists in the database. If not, it creates it. This is the first action the migrator performs.
          - `async fn get_applied_migrations(&self) -> anyhow::Result<HashSet<String>>`: Queries the `schema_migrations` table and returns a `HashSet` containing the filenames of all migrations that have already been run. A `HashSet` is efficient for checking existence.
          - `async fn apply_migration(&mut self, file_path: &Path) -> anyhow::Result<()>`: This function does two things in a single database **transaction**:
            1.  Reads the SQL content from the given file path.
            2.  Executes the SQL against the database.
            3.  Inserts the migration's filename into the `schema_migrations` table.

-----

### Mini Glossary

  - **`clap` Crate**: A powerful and popular Rust library for parsing command-line arguments. You'll use its `derive` feature to easily create a CLI structure from a simple Rust `struct`.
  - **`std::fs` Module**: Rust's standard library module for interacting with the filesystem. You'll use functions like `fs::read_dir` to find all your `.sql` migration files.
  - **Transaction**: A sequence of database operations performed as a single logical unit of work. If any operation within the transaction fails, all previous operations are rolled back. This is crucial to ensure that you don't record a migration as "applied" if the SQL script itself fails.
  - **`anyhow::Result`**: A flexible error handling utility. It allows you to easily propagate different kinds of errors (database errors, file I/O errors) up the call stack without needing to define custom error types, which is perfect for applications.

-----

### Timing & Accept criteria

  - **Time:** \~1 hour.

  - **Acceptance Checklist:**

    1.  Define the CLI structure using `clap`.
    2.  Implement logic to read all `.sql` filenames from a `./migrations` directory and sort them.
    3.  Implement the `Migrator` struct and its methods.
    4.  The tool must create the `schema_migrations` table on its first run against a fresh database.
    5.  The tool must correctly identify and execute only the pending migration files.
    6.  Each migration must be applied within a database transaction.
    7.  Running the tool a second time with no new `.sql` files should result in no database changes.
    8.  Demonstrate the full flow from the command line.

  - **Expected Output Snippets:**

      - First run with two migration files:
        ```sh
        $ cargo run -- migrate
        Ensuring migrations table exists...
        Found 2 migration files.
        Applying migration: 001_create_posts_table.sql...
        Applying migration: 002_create_comments_table.sql...
        ✅ All migrations applied successfully.
        ```
      - Second run with no new files:
        ```sh
        $ cargo run -- migrate
        Ensuring migrations table exists...
        Found 2 migration files.
        Database is up to date. No new migrations to apply.
        ```

-----

### Demo

Here's a snippet to get you started with the `main` function and `clap` setup. You'll need to create the `Migrator` and implement its logic.

```rust
use clap::Parser;
use std::path::{Path, PathBuf};
use tokio_postgres::{NoTls, Client};
use anyhow::Result;

/// A simple PostgreSQL migration tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Applies pending database migrations
    Migrate,
}

// TODO: Define the Migrator struct here.
// struct Migrator {
//     client: Client,
// }
//
// impl Migrator {
//     // TODO: Implement run, ensure_migrations_table, etc.
// }

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // TODO: Connect to your PostgreSQL database.
    // let (client, connection) = tokio_postgres::connect("...", NoTls).await?;
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("connection error: {}", e);
    //     }
    // });

    match cli.command {
        Commands::Migrate => {
            println!("Running migrations...");
            // TODO: Create a Migrator instance and call its `run` method.
        }
    }

    Ok(())
}

// Sample SQL file: migrations/001_create_posts_table.sql
/*
CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    body TEXT,
    published_at TIMESTAMPTZ DEFAULT NOW()
);
*/
```


Of course. Here is a Level-2 assignment for building a data import/export CLI tool.

### Title

CSV Manager – A CLI for Bulk Data Operations

### Description

  - The purpose of this assignment is to build a command-line tool for importing and exporting data between a PostgreSQL database and CSV files. The core logic will be handled by a `DataManager` struct.
  - Your tool will have two main functions: `import` and `export`. The `import` command will read records from a specified CSV file and insert them into a database table. The `export` command will do the reverse, writing all records from a table into a new CSV file.
  - The data flow for each operation is:

<!-- end list -->

```rust
// Import Flow:
// [user.csv] --read by--> [CLI App] --INSERT in transaction--> [PostgreSQL 'users' table]

// Export Flow:
// [PostgreSQL 'users' table] --SELECT *--> [CLI App] --write to--> [export.csv]
```

### Folder & File Structure

```
csv_manager/
├── data/
│   └── users_to_import.csv
└── src/
    └── main.rs
```

### Implementation details

  - **Main Type:** `DataManager`

  - **Internal Structure:**

      - `Cli`: A struct defined using the `clap` crate to handle command-line arguments. It should have `import` and `export` subcommands, each taking a file path as an argument.
      - `User`: A simple struct representing a user record with fields like `id` (integer, optional for creation), `name` (string), and `email` (string). It should derive `serde::Deserialize` and `serde::Serialize`.
      - `DataManager`: A struct that holds the database connection client (`tokio_postgres::Client`).

  - **Functions/Methods to Implement:**

      - `#[tokio::main] async fn main() -> anyhow::Result<()>`
          - **Explanation:** The entry point of your CLI tool. It will parse the command-line arguments, connect to the database, instantiate the `DataManager`, and call the appropriate method based on the subcommand (`import` or `export`).
      - `impl DataManager { ... }`
          - `async fn import(&mut self, file_path: &Path) -> anyhow::Result<()>`: This function reads a CSV file from the given path. It should parse each row into a `User` struct and insert all users into the `users` table. **Important**: All insertions should happen within a single database **transaction** so that the entire operation succeeds or fails together.
          - `async fn export(&self, file_path: &Path) -> anyhow::Result<()>`: This function queries the `users` table to fetch all records. It then uses a CSV writer to serialize each `User` record into a new CSV file at the specified path.
      - **Helper Function:** A function to ensure the `users` table exists. This can be called before any operation.

-----

### Mini Glossary

  - **`csv` Crate**: The standard Rust library for reading and writing CSV data. You'll use its `Reader` to deserialize file rows into your `User` struct and its `Writer` to serialize your `User` structs back into a file.
  - **`clap` Subcommands**: A feature of the `clap` crate that allows you to create multi-command CLIs, like `git push` or `cargo build`. In this assignment, `import` and `export` are subcommands.
  - **Transaction**: A sequence of database operations that are treated as a single, atomic unit. Using a transaction for the import ensures that if one record fails to insert, all previous insertions in that batch are rolled back, preventing a partially imported dataset.
  - **Serialization / Deserialization**: The process of converting a data structure (like your `User` struct) into a format for storage or transmission (like a CSV row or JSON) and vice-versa. The `serde` crate is the standard for this in Rust.

-----

### Timing & Accept criteria

  - **Time:** \~1 hour.

  - **Acceptance Checklist:**

    1.  Define the `Cli` and `User` structs.
    2.  Implement the `DataManager` with both `import` and `export` methods.
    3.  The `import` command must use a transaction and successfully load data from a sample CSV into the `users` table.
    4.  The `export` command must successfully write all data from the `users` table into a new CSV file.
    5.  The CLI must correctly parse subcommands and file path arguments.
    6.  Demonstrate both import and export operations from the command line.

  - **Expected Output Snippets:**

      - Running the import command:
        ```sh
        $ cargo run -- import data/users_to_import.csv
        Starting import from 'data/users_to_import.csv'...
        Successfully imported 3 records into the 'users' table.
        ```
      - Running the export command:
        ```sh
        $ cargo run -- export exports/all_users.csv
        Starting export to 'exports/all_users.csv'...
        Successfully exported 3 records.
        ```

-----

### Demo

This snippet provides the `clap` setup and the `main` function structure. Your task is to implement the `DataManager` and its methods.

```rust
use clap::Parser;
use std::path::{Path, PathBuf};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// A CLI tool to manage user data in PostgreSQL
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Import users from a CSV file into the database
    Import {
        /// The path to the CSV file to import
        #[arg(short, long)]
        path: PathBuf,
    },
    /// Export users from the database to a CSV file
    Export {
        /// The path where the CSV file will be saved
        #[arg(short, long)]
        path: PathBuf,
    },
}

// TODO: Define the User struct here with derive(Serialize, Deserialize)

// TODO: Define the DataManager struct and its `import` and `export` methods.

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // TODO: Establish connection to your PostgreSQL database.
    // let mut data_manager = DataManager::new(client).await?;

    match cli.command {
        Commands::Import { path } => {
            println!("Starting import from '{}'...", path.display());
            // TODO: Call data_manager.import(&path).await?;
        }
        Commands::Export { path } => {
            println!("Starting export to '{}'...", path.display());
            // TODO: Call data_manager.export(&path).await?;
        }
    }

    Ok(())
}

// Sample CSV file: data/users_to_import.csv
/*
name,email
Alice,alice@example.com
Bob,bob@example.com
Charlie,charlie@example.com
*/
```
