if let Some(category) = &filters.category {
    let has_none = category.values.iter().any(|c| c == "NONE");
    let other_ids: Vec<_> = category.values.iter().filter(|c| *c != "NONE").collect();

    if has_none {
        if other_ids.is_empty() {
            // Only NONE → empty categories
            where_conditions.push(format!("p.categories = ${}", params.len() + 1));
            params.push(&Vec::<String>::new());
        } else {
            // NONE + other IDs → empty OR overlap
            where_conditions.push(format!(
                "(p.categories = ${} OR p.categories && ${})",
                params.len() + 1,
                params.len() + 2
            ));
            params.push(&Vec::<String>::new());   // for empty []
            params.push(&other_ids);              // for other categories
        }
    } else {
        // Normal category filter
        where_conditions.push(format!("p.categories && ${}", params.len() + 1));
        params.push(&category.values);
    }
}