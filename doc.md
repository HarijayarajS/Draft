Of course. Here is the API guide for managing Privileges, following the professional format you provided.
API Guide: Privilege Management
This guide provides technical specifications for managing Privileges within the Enzo POS Access Control List (ACL) system. A Privilege represents the most granular, individual permission that can be assigned to a role. It defines a single action a user is allowed to perform, such as creating a product or deleting a staff member.
Managing these privileges is a core part of configuring the system's security and user capabilities. The typical workflow involves creating all necessary atomic permissions, which are then organized into groups and assigned to roles.
Base URL: https://api.enzo-pos.com/v1
Authentication: All endpoints require a Bearer <accessToken> in the Authorization header.
The Privilege Resource
This is the core data model that represents a single permission.
| Field | Type | Description | Constraints |
|---|---|---|---|
| id | Integer | The unique identifier for the privilege resource. | Read-only, Auto-generated |
| code | String | A unique, machine-readable key (e.g., product_create). | Required, Unique |
| title | String | A short, human-readable name (e.g., "Can create products"). | Required |
| description | String | A more detailed explanation of what the privilege allows. | Optional |
| identifier | String | A unique string identifier, often a kebab-case version of the code. | Required, Unique |
Endpoints ⚙️
Here are the available API endpoints for managing Privileges.
1. Create a Privilege
This endpoint registers a new atomic permission in the system.
 * Endpoint: POST /v1/privileges
 * Description: Creates a single, indivisible privilege that can later be assigned to groups and roles.
Request Body:
| Field | Type | Description |
|---|---|---|
| code | String | The unique, machine-readable key (e.g., "product_delete"). |
| title | String | The display name (e.g., "Can delete products"). |
| identifier | String | The unique string identifier (e.g., "product-can-delete"). |
| description | String | A detailed explanation of the permission. |
Example Request:
{
  "code": "product_delete",
  "title": "Can delete products",
  "identifier": "product-can-delete",
  "description": "Allows a user to permanently delete a product from the catalog."
}

Responses:
 * 200 OK: Success. The response body contains the id of the newly created resource.
   {
  "success": true,
  "error": null,
  "data": 505
}

 * 409 Conflict: Error. Returned if a privilege with the same code or identifier already exists.
2. List All Privileges
This endpoint retrieves a paginated list of all privileges.
 * Endpoint: POST /v1/privileges/list
 * Description: Returns a paginated and filterable list of all privileges, suitable for display in a management dashboard.
Request Body:
| Field | Type | Description |
|---|---|---|
| pageNo | Integer | The page number to retrieve. |
| rowsPerPage | Integer | The number of items to return per page. |
| keyword | String | A search term to filter results by title, code, or identifier. |
| sortKey | String | The field to sort by (e.g., "title", "code"). |
| sortType | String | The sort direction ("asc" or "desc"). |
Example Request:
{
  "pageNo": 1,
  "rowsPerPage": 10,
  "sortKey": "title",
  "sortType": "asc",
  "keyword": "product"
}

Responses:
 * 200 OK: Success. The response body contains an array of privilege objects and pagination details.
   {
  "success": true,
  "error": null,
  "data": {
    "items": [
      {
        "id": 2,
        "code": "product_create",
        "title": "Can add product",
        "description": "User can add new products",
        "identifier": "product-can-add"
      }
    ],
    "pagination": {
      "pageNo": 1,
      "totalRows": 1,
      "rowsPerPage": 10
    }
  }
}

3. Get a Specific Privilege
This endpoint retrieves detailed information for a single privilege by its unique id.
 * Endpoint: GET /v1/privileges/{id}
 * Description: Fetches the details of one privilege by its ID.
Path Parameters:
| Parameter | Type | Description |
|---|---|---|
| id | Integer | The unique ID of the privilege. |
Responses:
 * 200 OK: Success. The response body contains the full privilege object.
   {
  "success": true,
  "error": null,
  "data": {
    "id": 505,
    "code": "staff_can_add_member",
    "title": "Can add member",
    "description": "User can able to add the member in staff",
    "identifier": "staff-can-add-member"
  }
}

 * 202 Accepted / 404 Not Found: Error. Returned if no privilege with the specified id exists.
4. Update a Privilege 📝
This endpoint updates one or more properties of an existing privilege.
 * Endpoint: PATCH /v1/privileges/{id}
 * Description: Supports partial or full updates. Only include the fields you wish to change. The code and identifier must remain unique if updated.
Path Parameters:
| Parameter | Type | Description |
|---|---|---|
| id | Integer | The unique ID of the privilege to update. |
Example Request (Partial Update):
{
  "title": "Can Add New Staff Member",
  "description": "Allows a user to create a new staff profile in the organization."
}

Responses:
 * 200 OK: Success. Indicates the update was successful.
   {
  "success": true,
  "error": null
}

 * 202 Accepted / 404 Not Found: Error. Returned if no privilege with the specified id exists.
5. Delete a Privilege 🗑️
This endpoint permanently deletes a privilege.
 * Endpoint: DELETE /v1/privileges/{id}
 * Description: This action is irreversible and will automatically unlink the privilege from any groups or roles it was assigned to. Use with caution.
Path Parameters:
| Parameter | Type | Description |
|---|---|---|
| id | Integer | The unique ID of the privilege to delete. |
Responses:
 * 200 OK: Success. Indicates the resource was deleted.
   {
  "success": true,
  "error": null
}

 * 202 Accepted / 404 Not Found: Error. Returned if no privilege with the specified id exists.


Of course. Here is the API guide for managing Roles, following the professional format you requested.
API Guide: Role Management
This guide provides technical specifications for managing Roles within the Enzo POS Access Control List (ACL) system. A Role is a named collection of individual privileges (e.g., "Manager," "Cashier," "Kitchen Staff") that can be assigned to staff members to grant them specific permissions.
Managing roles is the final step in connecting users to permissions. The typical workflow involves fetching the available privilege hierarchy, creating a role with a selection of those privileges, and then assigning that role to one or more staff members.
Base URL: https://api.enzo-pos.com/v1
Authentication: All endpoints require a Bearer <accessToken> in the Authorization header.
The Role Resource
This is the core data model that represents a user role. The resource contains basic details and a nested tree of all available privileges, indicating which ones are selected for that role.
| Field | Type | Description | Constraints |
|---|---|---|---|
| id | Integer | The unique identifier for the role resource. | Read-only, Auto-generated |
| code | String | A unique, machine-readable key (e.g., manager_store_a). | Optional, Unique |
| title | String | A human-readable name for the role displayed in the UI. | Required |
| isActive | Boolean | A flag to enable or disable the role. | Optional, Defaults to true |
| applications | Array | A hierarchical tree of all applications, groups, and privileges, with isSelected flags indicating which privileges are granted to this role. | Returned on GET /v1/roles/{id} |
Endpoints ⚙️
Here are the available API endpoints for managing Roles.
1. Get Privilege Tree for Role Creation
This special endpoint is the first step for creating or editing a role. It retrieves the entire hierarchy of all available privileges, organized by applications and groups, which you can then display in a UI for selection.
 * Endpoint: GET /v1/roles/privilege-details
 * Description: Fetches the complete permission tree. All isSelected flags will be false by default.
Responses:
 * 200 OK: Success. The response body contains a hierarchical array of application objects, each containing groups, sub-groups, and privileges.
   {
  "success": true,
  "error": null,
  "data": [
    {
      "id": 18,
      "code": "dashboard",
      "title": "Dashboard",
      "isSelected": false,
      "groups": [
        {
          "id": 19,
          "code": "inventory",
          "title": "Inventory",
          "isSelected": false,
          "subGroups": [
            {
              "id": 22,
              "code": "product",
              "title": "Product",
              "isSelected": false,
              "privileges": [
                {
                  "id": 2,
                  "code": "product_create",
                  "title": "Can add product",
                  "description": "User can add new products",
                  "isSelected": false
                }
              ]
            }
          ],
          "privileges": []
        }
      ]
    }
  ]
}

2. Create a Role
This endpoint creates a new role with a specified set of privileges.
 * Endpoint: POST /v1/roles
 * Description: Creates a new role. The request body requires a flat array of privilegeIds that were selected from the privilege tree.
Request Body:
| Field | Type | Description |
|---|---|---|
| title | String | The display name for the role (e.g., "Store Manager"). |
| code | String | The unique, machine-readable key (e.g., "store_manager"). |
| privilegeIds | Array of Integers | A flat list of unique privilege IDs to grant to this role. |
Example Request:
{
  "title": "Store Manager",
  "code": "store_manager",
  "privilegeIds": [2, 8, 18]
}

Responses:
 * 200 OK: Success. The response body contains the id of the newly created role.
   {
  "success": true,
  "error": null,
  "data": 505
}

3. List All Roles
This endpoint retrieves a paginated list of all roles in the organization.
 * Endpoint: POST /v1/roles/list
 * Description: Returns a paginated and filterable list of roles.
Request Body:
| Field | Type | Description |
|---|---|---|
| pageNo | Integer | The page number to retrieve. |
| rowsPerPage | Integer | The number of items per page. |
| filter.isActive | Boolean | Filters roles by their active status. |
| keyword | String | A search term to filter by title or code. |
Example Request:
{
  "pageNo": 1,
  "rowsPerPage": 20,
  "filter": {
    "isActive": true
  }
}

Responses:
 * 200 OK: Success. The response body contains an array of role objects and pagination details.
   {
  "success": true,
  "error": null,
  "data": {
    "items": [
      {
        "id": 17,
        "code": "manager_store_a",
        "title": "Manager",
        "isActive": true
      }
    ],
    "pagination": {
      "pageNo": 1,
      "totalRows": 1,
      "rowsPerPage": 20
    }
  }
}

4. Get a Specific Role
This endpoint retrieves the detailed information for a single role, including its assigned privileges.
 * Endpoint: GET /v1/roles/{id}
 * Description: Fetches one role by its ID. The response includes the full permission tree where the isSelected flag is true for all privileges granted to this role.
Path Parameters:
| Parameter | Type | Description |
|---|---|---|
| id | Integer | The unique ID of the role. |
Responses:
 * 200 OK: Success. The response body contains the full role object with its selected privileges.
   {
  "success": true,
  "error": null,
  "data": {
    "code": "manager_store_a",
    "title": "Manager",
    "isActive": true,
    "applications": [
      {
        "id": 18,
        "code": "dashboard",
        "title": "Dashboard",
        "isSelected": true,
        "groups": [
          {
            "id": 22,
            "code": "product",
            "title": "Product",
            "isSelected": true,
            "privileges": [
              {
                "id": 2,
                "code": "product_create",
                "title": "Can add product",
                "isSelected": true // This privilege is granted to the role
              }
            ]
          }
        ]
      }
    ]
  }
}

 * 202 Accepted / 404 Not Found: Error. Returned if no role with the specified id exists.
5. Update a Role 📝
This endpoint updates one or more properties of an existing role.
 * Endpoint: PATCH /v1/roles/{id}
 * Description: Supports partial updates. To change permissions, provide the complete new list of privilegeIds.
Path Parameters:
| Parameter | Type | Description |
|---|---|---|
| id | Integer | The unique ID of the role to update. |
Example Request (Partial Update):
{
  "title": "Head Manager",
  "privilegeIds": [2, 8, 18, 21]
}

Responses:
 * 200 OK: Success. Indicates the update was successful.
   {
  "success": true,
  "error": null
}

 * 202 Accepted / 404 Not Found: Error. Returned if no role with the specified id exists.
6. Delete a Role 🗑️
This endpoint permanently deletes a role.
 * Endpoint: DELETE /v1/roles/{id}
 * Description: This action is irreversible. Staff members assigned to this role will lose their permissions and may need to be reassigned to a new role.
Path Parameters:
| Parameter | Type | Description |
|---|---|---|
| id | Integer | The unique ID of the role to delete. |
Responses:
 * 200 OK: Success. Indicates the resource was deleted.
   {
  "success": true,
  "error": null
}

 * 202 Accepted / 404 Not Found: Error. Returned if no role with the specified id exists.
