# API Handler Guidelines

## 1. Response Types

### Response Structures
| Type               | Fields                               |
|--------------------|----------------------------------------|
| **CommonResponse** | `success`, `error`                     |
| **DataResponse**   | `success`, `error`, `data`             |
| **FileResponse**   | `path`, `url`, `name`, `size`          |

---

## 2. Status Code Mapping

### Response Codes
| Action / Case       | Code | Description                              |
|---------------------|------|------------------------------------------|
| **Updated**         | 200  | Updated successfully in DB               |
| **Delete**          | 200  | Soft or permanent deletion               |
| **Get**             | 200  | Data retrieved successfully              |
| **Create**          | 201  | New record inserted                      |
| **Accepted Error**  | 202  | Business/logic-level error               |
| **Bad Request**     | 400  | Invalid JSON payload                     |
| **Unauthorized**    | 401  | Invalid or expired token                 |
| **Forbidden**       | 403  | No permission for the action             |
| **Not Found**       | 404  | Requested data does not exist            |
| **Internal Error**  | 500  | Unexpected query or server failure       |

---

## 3. Method & Action Mapping

| Method    | Action           | Description                                 |
|-----------|------------------|---------------------------------------------|
| **GET**   | `get`            | When there is no payload                    |
| **POST**  | `create`, `get`  | When there is a JSON payload                |
| **PUT**   | `update`         | Update with JSON payload                    |
| **DELETE**| `delete`         | Delete an item                              |
| **ANY**   | `ws`             | For WebSocket connections                   |

---

## 4. Handler Guidelines

- API calls requiring **user information** must be placed **above the Middleware**.
- Each handler must log incoming parameters using **`log_args::params`**.
- Handlers must return the appropriate **HTTP status code** based on the API action.
- For `org` and `venue` scoped APIs:
  - Perform validation before processing.
  - Return appropriate error response on validation failure.

---

## 5. Logging Standards

### Logging Levels
- Use **`debug!`** for normal handler logs.
- Use **`error!`** for failures or critical issues.

### Log Examples

#### Getting user info
- Start:  
  `Fetching user information`
- End:  
  `Fetched the user information successfully`

#### Grid API example
- Start:  
  `Fetching the category grid with options {"keyword": "cat"}`
- End:  
  `Fetched 10 category items`

---

## 6. Authentication

- All API calls must use **Bearer token** authentication.
- Provided via the `Authorization` header:

```
Authorization: Bearer <token>
```

---


### Guidelines

---

#### Query

- Always verify that the item **exists in the database** before performing any operation.
- Every query that uses `WHERE` conditions should follow this order:
  1. `org_id`
  2. `venue_id`
  3. Other IDs (doc_id, user_id, etc.)
  4. Flags (`is_deleted`, `is_archived`)
  5. `ANY()` filters
  6. Text searches using `ILIKE`
- Queries expected to return **more than one record** should always include:
  - `ORDER BY created_on`  
  - or a relevant indexed column
- Use **prepared statements with parameters** for all queries (never concatenate dynamic values).
- Always use a **transaction** when:
  - Multiple tables need to be updated  
  - Multiple steps depend on earlier steps
- Avoid `query_one`; always prefer **`query_opt`** for safer optional response handling.
- Use **`LIMIT`** in list/grid APIs to avoid uncontrolled result sets.
- Use **`OFFSET` + LIMIT` only for simple pagination**, else prefer keyset pagination.
- For delete operations:
  - Prefer soft-delete (`is_deleted = true`) unless explicitly required.
  - Always update `modified_on` and `synced_on` if applicable.
- For update calls:
  - Only update changed fields when possible to improve performance.
- Always check row count (`rows_affected`) for update and delete queries to verify success.
- Prefer **CTEs** (`WITH ...`) for complex multi-step queries.
- Use proper **indexes** for:
  - `org_id`, `venue_id`
  - `doc_id`
  - `is_deleted`
  - frequently searched fields
- Use `FOR UPDATE` lock only when needed (preventing deadlocks).

---

#### Query Format

##### **INSERT**
```sql
INSERT INTO account (
    doc_id,
    org_id,
    balance,
    ...
) VALUES (
    $1, $2, $3, ...$10,
    $11, ...
) RETURNING id;
```

##### **UPDATE**
```sql
UPDATE account SET
    ...,
    modified_on = NOW(),
    synced_on = NOW()
WHERE doc_id = $1 
  AND org_id = $2;
```

##### **DELETE (Soft Delete Example)**
```sql
UPDATE account SET
    is_deleted = TRUE,
    modified_on = NOW()
WHERE doc_id = $1 
  AND org_id = $2;
```

---

#### Function

- Struct validation must be performed **at the start** of the function.
- Always validate:
  - Required fields
  - Field lengths
  - Allowed enums
  - Numeric limits
  - Cross-field rules (e.g., start < end)
- Return early on validation errors (avoid deep nesting).
- Use strong typing (e.g., `Uuid`, `i64`, `chrono::DateTime`) instead of strings.
- Avoid passing raw IDs; use meaningful types (e.g., `OrgId`, `VenueId` newtypes).
- Every function should log:
  - Start event
  - Important parameters (masked where needed)
  - End event with success or error
- Keep functions single-purpose (SRP).
- Avoid long functions; split into smaller helpers.
- Avoid duplication — prefer reusable validation and query helpers.
- Always sanitize and normalize user text inputs (trim, lowercase if needed).

---

#### Additional Best Practices

- Use **ISO timestamps** (`TIMESTAMPTZ`) everywhere for consistency.
- Always convert empty strings to `NULL` when required.
- Prefer `snake_case` column names to maintain uniformity.
- For boolean flags, use meaningful names (`is_active`, `is_deleted`).
- Avoid storing computed fields; compute them at query time if lightweight.
- Avoid using `SELECT *`; always list required columns explicitly.
- Use strong constraints in DB:
  - `NOT NULL`
  - Foreign keys
  - Default values
- Prefer database-driven validations when possible (unique constraints, check constraints).

---
