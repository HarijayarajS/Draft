/// To get the product count that match the given rules
pub async fn get_smart_rules(
    db: &DBConnection<'_>,
    org_id: &str,
    input: &AttributeSmartRulesFilter,
) -> ApiResult<AttributeSmartRulesMatches> {
    // --- CHANGED: Added "none" to valid types ---
    if !matches!(input.apply_type.as_str(), "all" | "any" | "none") {
        return Err(ApiError::Error("Invalid apply type".to_string()));
    }

    // If no filters given return 0
    if input.rules.is_empty() {
        return Ok(AttributeSmartRulesMatches {
            no_matches: 0,
            products: vec![],
        });
    }

    let mut query = "SELECT p.doc_id, p.title, p.categories FROM product p ".to_string();
    let mut where_clauses: Vec<String> = Vec::new();
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

    let mut to_match_categories = Vec::new();
    let mut to_ignore_categories = Vec::new();

    let mut to_match_location = Vec::new();
    let mut to_ignore_location = Vec::new();

    for rule in input.rules.iter() {
        if rule.object == "category" {
            if rule.condition == "match" {
                to_match_categories.push(rule.value.clone());
            } else {
                to_ignore_categories.push(rule.value.clone());
            }
        } else if rule.object == "location" {
            if rule.condition == "match" {
                to_match_location.push(rule.value.clone());
            } else {
                to_ignore_location.push(rule.value.clone());
            }
        }
    }

    if !to_ignore_location.is_empty() || !to_match_location.is_empty() {
        query.push_str(" JOIN venue_product vp ON vp.product_id = p.doc_id ");

        if !to_match_location.is_empty() {
            where_clauses.push(format!("vp.venue_id = ANY(${})", params.len() + 1));
            params.push(&to_match_location);
        }

        if !to_ignore_location.is_empty() {
            where_clauses.push(format!(
                "vp.venue_id NOT IN (SELECT UNNEST(${}::text[]))",
                params.len() + 1
            ));
            params.push(&to_ignore_location);
        }
    }

    if !to_match_categories.is_empty() {
        // --- CHANGED: "all" is the special case, "any" and "none" use '&&' ---
        if input.apply_type == "all" {
            // "all": Product must contain ALL specified categories. Use '@>' operator.
            where_clauses.push(format!("p.categories @> ${}", params.len() + 1));
        } else {
            // "any" or "none": Product must contain ANY of the specified categories. Use '&&' operator.
            // For "none", we will negate the whole block of clauses later.
            where_clauses.push(format!("p.categories && ${}", params.len() + 1));
        }
        params.push(&to_match_categories);
    }

    if !to_ignore_categories.is_empty() {
        // This logic is correct: NOT (overlaps) means it must not contain ANY of the ignored categories.
        where_clauses.push(format!("NOT (p.categories && ${})", params.len() + 1));
        params.push(&to_ignore_categories);
    }

    if where_clauses.is_empty() {
        return Ok(AttributeSmartRulesMatches {
            no_matches: 0,
            products: vec![],
        });
    }

    query.push_str(" WHERE ");

    // --- CHANGED: Added block for "none" ---
    if input.apply_type == "none" {
        // "none": Negate the "any" logic (NOT (rule1 OR rule2 OR ...))
        query.push_str(&format!("NOT ({})", &where_clauses.join(" OR ")));
    } else if input.apply_type == "any" {
        // "any": (rule1 OR rule2 OR ...)
        query.push_str(&format!("({})", &where_clauses.join(" OR ")));
    } else if input.apply_type == "all" {
        // "all": (rule1 AND rule2 AND ...)
        query.push_str(&format!("({})", &where_clauses.join(" AND ")));
    }
    // --- END CHANGE ---


    // Default org id and active products where query
    query.push_str(&format!(
        " AND p.org_id = ${} AND p.is_deleted = 'f' AND p.is_archive = 'f'",
        params.len() + 1
    ));
    params.push(&org_id);

    // Adding group by to handle multiple venue rows for a single product
    query.push_str(" GROUP BY p.doc_id, p.title, p.categories");
    
    // Adding order by
    query.push_str(" ORDER BY p.created_on");


    let mut category_ids: HashSet<String> = HashSet::new();
    let mut categories: HashMap<String, String> = HashMap::new();
    let mut products: Vec<AttributeSmartRulesProductInfo> = Vec::new();

    let rows = db.query(&query, &params).await?;
    for row in rows {
        let categories: Vec<OptionItemString> = row
            .get_as_vec_string("categories")
            .iter()
            .map(|id| {
                category_ids.insert(id.to_string());
                OptionItemString {
                    id: id.to_string(),
                    title: String::new(),
                }
            })
            .collect();

        products.push(AttributeSmartRulesProductInfo {
            id: row.get("doc_id"),
            title: row.get("title"),
            categories,
        });
    }

    let categories_vec: Vec<String> = category_ids.iter().map(|id| id.to_string()).collect();
    let category_row = db
        .query(
            "SELECT doc_id, title FROM category  
            WHERE doc_id = ANY($1) AND org_id = $2
            ORDER BY title",
            &[&categories_vec, &org_id],
        )
        .await?;
    for row in category_row {
        categories.insert(row.get_as_string("doc_id"), row.get_as_string("title"));
    }

    // Mapping category titles
    for product in products.iter_mut() {
        for category in product.categories.iter_mut() {
            category.title = categories
                .get(&category.id)
                .map_or("", |val| val)
                .to_string();
        }
    }

    Ok(AttributeSmartRulesMatches {
        no_matches: products.len(),
        products,
    })
}
