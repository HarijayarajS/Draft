use std::fs;

fn count_rs_files(path: &str) -> std::io::Result<usize> {
    Ok(fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
        .count())
}



WITH RECURSIVE category_path AS (
    SELECT
        doc_id,
        name,
        parent_id,
        doc_id AS start_doc_id,
        1 AS depth
    FROM category
    WHERE doc_id = ANY($1)
      AND org_id = $2
      AND is_deleted = false

    UNION ALL

    SELECT
        c.doc_id,
        c.name,
        c.parent_id,
        cp.start_doc_id,
        cp.depth + 1
    FROM category c
    INNER JOIN category_path cp
        ON c.doc_id = cp.parent_id
    WHERE c.org_id = $2
      AND c.is_deleted = false
)

SELECT
    start_doc_id,
    name
FROM category_path
ORDER BY start_doc_id, depth DESC;