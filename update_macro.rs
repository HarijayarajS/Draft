use serde_json::Value;
use tokio_postgres::types::ToSql;

// Fetch the category doc
let row = db
    .query_opt(
        "SELECT doc FROM category WHERE doc_id = $1 AND company_id = $2",
        &[&category_id, &company_id],
    )
    .await?
    .ok_or_else(|| ApiError::Error("Invalid category id".to_string()))?;

let mut doc: CategoryRowDoc = row.get_as_struct("doc")?;

// Update doc fields only if needed
macro_rules! update_field {
    ($opt:expr, $target:ident) => {
        if let Some(value) = &$opt {
            doc.$target = value.clone();
        }
    };
}

update_field!(input.icon, icon);
update_field!(input.image, image);
update_field!(input.info, info);
update_field!(input.extral, extral);
update_field!(input.extra2, extra2);
update_field!(input.extra3, extra3);
if let Some(list) = &input.sort_info {
    doc.sort_info = list.clone();
}
if let Some(list) = &input.platforms {
    doc.platforms = list.clone();
}
if let Some(val) = input.is_web_online {
    doc.is_web_online = val;
}

// Start building SET clause
let mut set_clauses = vec!["modified_on = current_timestamp".to_string()];
let mut params: Vec<&(dyn ToSql + Sync)> = vec![];

// Always update doc if anything changed
let doc_value = serde_json::to_value(&doc)?;
set_clauses.push(format!("doc = ${}", params.len() + 1));
params.push(&doc_value);

// Optional fields (simple macro to reduce repetition)
macro_rules! opt_param {
    ($field:expr, $name:literal) => {
        if let Some(val) = &$field {
            set_clauses.push(format!("{} = ${}", $name, params.len() + 1));
            params.push(val);
        }
    };
}

opt_param!(input.title, "title");
opt_param!(input.enabled_platforms, "enabled_platforms");
opt_param!(input.printers, "printers");
opt_param!(input.time_slot_id, "time_slot_id");
opt_param!(input.parent_id, "parent_id");
opt_param!(input.user_tags, "user_tags");

// Add WHERE clause values
let doc_id_idx = params.len() + 1;
params.push(&category_id);

let company_id_idx = params.len() + 1;
params.push(&company_id);

// Final SQL
let query = format!(
    "UPDATE category SET {} WHERE doc_id = ${} AND company_id = ${}",
    set_clauses.join(", "),
    doc_id_idx,
    company_id_idx
);

// Execute update
let affected = db.execute(&query, &params).await?;
Ok(affected != 0)