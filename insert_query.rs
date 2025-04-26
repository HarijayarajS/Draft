fn create_insert_from_create_table(create_table_sql: &str) -> Option<String> {
    // Make sure it's case-insensitive
    let sql = create_table_sql.to_lowercase();
    
    // Find table name
    let table_start = sql.find("table if not exists")?;
    let rest = &sql[table_start + "table if not exists".len()..];
    let mut parts = rest.trim().splitn(2, '(');
    
    let table_name = parts.next()?.trim();
    let fields_part = parts.next()?.trim();
    
    // Remove the trailing ')' if it exists
    let fields_part = fields_part.strip_suffix(')')?.trim();
    
    // Split fields
    let fields: Vec<&str> = fields_part
        .split(',')
        .map(|f| f.trim().split_whitespace().next().unwrap_or(""))
        .filter(|&f| !f.is_empty())
        .collect();
    
    // Generate placeholders: $1, $2, ...
    let placeholders: Vec<String> = (1..=fields.len())
        .map(|i| format!("${}", i))
        .collect();
    
    // Form the INSERT statement
    let insert_sql = format!(
        "INSERT INTO {} ({}) VALUES ({});",
        table_name,
        fields.join(", "),
        placeholders.join(", ")
    );
    
    Some(insert_sql)
}