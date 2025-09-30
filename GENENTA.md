# 📖 Genenta – Training Guide Overview

1. **Page 1: Introduction to PostgreSQL with Rust**  
   Explains why PostgreSQL is a strong choice for Rust backends, covers setup steps, and introduces async database handling in Rust.  

2. **Page 2: Connecting to PostgreSQL**  
   Shows how to establish database connections using `tokio-postgres` and `SQLx`, including configuration and connection pooling.  

3. **Page 3: Basic Queries – SELECT, INSERT, UPDATE, DELETE**  
   Demonstrates core SQL operations in Rust with examples for querying, inserting, updating, and deleting rows.  

4. **Page 4: Using Query Parameters**  
   Introduces parameterized queries to prevent SQL injection and manage dynamic values safely.  

5. **Page 5: Async & Await with Queries**  
   Explains how async queries work in Rust, how to await results, and handle concurrency with PostgreSQL.  

6. **Page 6: Connection Pooling**  
   Covers why pooling is important, how to configure pools with SQLx, and best practices for managing multiple connections.  

7. **Page 7: Prepared Statements and Reuse**  
   Shows how to prepare SQL statements once and reuse them for performance and safety in high-throughput applications.  

8. **Page 8: Mapping Rows to Rust Structs**  
   Explains how to map SQL rows into strongly-typed Rust structs, using `FromRow` and derive macros for cleaner code.  

9. **Page 9: Handling Common Errors and Safe Retrying**  
   Covers common database errors (timeouts, unique violations, disconnects) and strategies for retries with exponential backoff.  

10. **Page 10: Advanced Techniques – Batch Operations & Transactions Optimization**  
    Demonstrates batch inserts/updates, transaction handling, and optimizing for performance in real-world apps.  

11. **Page 11: Building a Reusable Database Layer / Abstraction**  
    Guides how to create a clean, maintainable abstraction layer for database queries, separating logic from raw SQL.  

12. **Page 12: Best Practices & Advanced Topics**  
    Summarizes industry best practices: migrations, testing with databases, performance tuning, and security considerations.  
