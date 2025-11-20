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