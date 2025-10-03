SELECT COUNT(DISTINCT p.id) AS product_count
FROM product p
JOIN venue_product vp ON vp.product_id = p.id
WHERE vp.venue_id = ANY($5::text[])                  -- only these venues are supported
  AND vp.venue_id NOT IN (SELECT unnest($3::text[])) -- ignore these venues
  AND p.categories && $1::text[]                     -- must match at least one category
  AND NOT (p.categories && $2::text[])               -- must NOT match excluded categories
  AND NOT (p.categories && $4::text[]);              -- completely ignore these categories