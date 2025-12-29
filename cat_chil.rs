pub async fn get_all_child_ids(
    db: &DB,
    org_id: &str,
    parent_ids: &Vec<String>,
) -> ApiResult<Vec<String>> {

    let rows = db
        .query(
            r#"
            WITH RECURSIVE category_tree AS (
                SELECT doc_id, parent_id
                FROM category
                WHERE parent_id = ANY($1)
                  AND org_id = $2
                  AND is_deleted = false

                UNION ALL

                SELECT c.doc_id, c.parent_id
                FROM category c
                JOIN category_tree ct ON c.parent_id = ct.doc_id
                WHERE c.is_deleted = false
            )
            SELECT DISTINCT doc_id FROM category_tree
            "#,
            &[parent_ids, &org_id],
        )
        .await?;

    Ok(rows.into_iter().map(|r| r.get("doc_id")).collect())
}