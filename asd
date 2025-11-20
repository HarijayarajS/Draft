## Handler guidelines

### Response types
- CommonResponse -> (success, error)
- DataResponse -> (success, error, data)
- FileResponse -> (path, url, name, size)

#### Response with Status Code
| Action                | Code | Description                      |
| --------------------- | ---- | -------------------------------- |
| Updated               | 200  | Updated in db                    |
| Delete                | 200  | Soft or permanent deletion in db |
| Get                   | 200  | Retrieved data                   |
| Create                | 201  | Inserted in db                   |
| Accepted error        | 202  | For logic error                  |
| Bad Request           | 400  | Invalid payload json             |
| Unauthorized          | 401  | Invalid or expired token         |
| Forbidden             | 403  | Not have access for an action    |
| Not exist             | 404  | If requested data not exist      |
| Internal server error | 500  | Un expected query or any error   |

#### Method And actions
| Method | Action      | Description                           |
| ------ | ----------- | ------------------------------------- |
| GET    | get         | If there is no payload                |
| POST   | Create, Get | If there is JSON payload              |
| PUT    | Update      | For update when there is JSON payload |
| DELETE | Delete      | For delete calls                      |
| ANY    | Ws          | For using websocket connection        |

### Guidelines
#### Handler
- Every calls that need user information should to placed above te `Middleware`
- Each handler should have `log_args::params` to log the request
- According to the `API` calls create, update, delete or get should return the appropriate status code
- Validation for `org` and `venue` level calls should be done and return error if any

#### Logging formate
- Each Handler should use the `debug!` level log for logging
- For error use `error!` level

> Example log for getting user info
- At start -> 'Fetching user information'
- At end -> 'Fetched the user information successfully'

> Example log for grid calls
- At start -> 'Fetching the category grid with options `{"keyword" : "cat"}`'
- At end -> 'Fetched 10 category items'

### Auth
- Every calls can be accessed with `Bearer` token formate in the `Authorization` header

## Domain Api guidelines

### Error types
- NotFound -> if the request item not exist in db
- Error -> For logical error
- And other required error types

### Guidelines
#### Query
- Always verify the item is exist in the db before any operation
- Every query that uses `WHERE` conditions should follow with order
  - org_id
  - venue_id
  - other_ids if exist
  - Flags (is_deleted, is_archived)
  - ANY()
  - Ilike

- If a query return more than one row that should use `ORDER BY`
- Use statement and params for all querying
- Always use transaction for More than one table changes
- Avoid using `query_one`, use `query_opt`

#### Query Formate
> INSERT
```sql
    INSERT INTO account (
            doc_id,
            org_id,
            balance,
            ...
        ) VALUES (
            $1, $2, $3, ...$10
            $11, ...
        ) RETURNING id
```
> UPDATE
```sql
    UPDATE account SET
    ...
    modified_on = NOW(),
    synced_on = NOW()
    WHERE doc_id = $1 AND org_id = $2",
```

#### Function
- Struct validation fields should to be validated at the start
