You can query inside a JSONB array of objects in PostgreSQL using jsonb_array_elements, the @> containment operator, or the ? operators depending on your need.

Assume your table:

table: orders
column: doc (jsonb)

And inside doc you have:

{
  "paymentTypes": [
    { "id": "p1", "title": "Card" },
    { "id": "p2", "title": "Cash" }
  ]
}


---

✅ 1. Search by title inside paymentTypes (exact match)

SELECT *
FROM orders
WHERE doc->'paymentTypes' @> '[{"title": "Cash"}]';

This returns rows having at least one element with "title": "Cash".


---

✅ 2. Case-insensitive search (ILIKE)

SELECT *
FROM orders
WHERE EXISTS (
    SELECT 1
    FROM jsonb_array_elements(doc->'paymentTypes') elem
    WHERE elem->>'title' ILIKE '%cash%'
);


---

✅ 3. Search partial match using ILIKE on the array

SELECT *
FROM orders
WHERE EXISTS (
    SELECT 1
    FROM jsonb_array_elements(doc->'paymentTypes') p
    WHERE p->>'title' ILIKE '%card%'
);


---

✅ 4. Search by id inside array

SELECT *
FROM orders
WHERE doc->'paymentTypes' @> '[{"id": "p1"}]';


---

✅ 5. If paymentTypes might NOT exist

To avoid NULL errors:

SELECT *
FROM orders
WHERE EXISTS (
    SELECT 1
    FROM jsonb_array_elements(COALESCE(doc->'paymentTypes', '[]'::jsonb)) elem
    WHERE elem->>'title' ILIKE '%cash%'
);


---

✅ 6. Index suggestion (for speed)

CREATE INDEX idx_orders_doc_gin ON orders USING gin (doc jsonb_path_ops);


---

Bonus: Search by title OR id together

SELECT *
FROM orders
WHERE EXISTS (
    SELECT 1
    FROM jsonb_array_elements(doc->'paymentTypes') p
    WHERE p->>'title' ILIKE '%cash%'
       OR p->>'id' = 'p1'
);


---

If you want, I can provide:

✅ fuzzy search
✅ pattern search
✅ index optimization
✅ how to avoid slow queries

Just tell me!