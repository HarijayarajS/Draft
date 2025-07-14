Here’s a clean and structured version of your WebSocket event subscription system, including a sample message format, behavior description, trigger function responsibility, and event dispatch flow. This can be shared with your manager or used for implementation:


---

📘 WebSocket Event Subscription System - Specification

🔁 Client → Server Subscription Request

{
  "event": "subscribe",
  "collection": "category",
  "filter": {
    "isDeleted": true,
    "isArchive": true,
    "keyword": "Water",
    "createdOn": {
      "from": 123213213,
      "to": 3214124213
    },
    "printers": {
      "values": ["6593885346766956fbfe773b"]
    },
    "timeSlotId": {
      "value": "94699e3d118245d3a135fe08163241c7"
    },
    "noProducts": {
      "from": 123213213,
      "to": 3214124213
    }
  }
}

Authorization header must include a valid token to extract companyId.

collection defines the data type (e.g., "category") to monitor.

filter is an optional field, providing fine-grained filtering.



---

📥 Server-Side Behavior on Subscription

1. Validate Authorization Token
Extract companyId from the JWT token provided in the header.


2. Parse Subscription Message

Parse the collection and filter.

Store the WebSocket connection along with its filters (in-memory or in a lightweight broker).



3. Subscribe to DB Events

Use PostgreSQL LISTEN/NOTIFY or custom triggers on the table category.



4. On DB Event Trigger:

Receive event from PostgreSQL for the category table (created, updated, deleted).

Call a trigger function to return all searchable fields.





---

🧠 Trigger Function Structure (Database Side)

Should be attached to INSERT, UPDATE, and DELETE on the category table.

-- Pseudocode for PostgreSQL function
CREATE FUNCTION notify_category_change() RETURNS trigger AS $$
DECLARE
  payload JSON;
BEGIN
  payload := json_build_object(
    'id', NEW.id,
    'company_id', NEW.company_id,
    'created_on', NEW.created_on,
    'title', NEW.title,
    'is_deleted', NEW.is_deleted,
    'is_archive', NEW.is_archive,
    'no_products', NEW.no_products,
    'parent_id', NEW.parent_id,
    'printers', NEW.printers,
    'user_tags', NEW.user_tags,
    'enabled_platforms', NEW.enabled_platforms,
    'time_slot_id', NEW.time_slot_id,
    'event', TG_OP
  );
  
  PERFORM pg_notify('category_events', payload::text);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;


---

📤 Server Sends Back to Client (Only If Filters Match)

Client Response Format:

{
  "id": "docId",
  "event": "created" | "updated" | "deleted"
}

Server validates DB event data against the active filter.

If match → send above event to only those subscribed clients.



---

📌 Important Notes

✅ Only one active subscription per WebSocket connection

✅ Company ID-based scoping of data

⚠️ Trigger function returns all searchable fields needed for matching

✅ No need to store filters in DB – kept in memory per connection

✅ Supports dynamic filtering logic (e.g., range, set, value match)



---

📊 Example Filter Match Logic (Rust Server Pseudocode)

fn matches_filter(event_data: &CategoryEvent, filter: &CategoryFilter) -> bool {
    filter.is_deleted.map_or(true, |v| v == event_data.is_deleted)
    && filter.is_archive.map_or(true, |v| v == event_data.is_archive)
    && filter.keyword.as_ref().map_or(true, |k| event_data.title.contains(k))
    && filter.created_on.map_or(true, |range| event_data.created_on >= range.from && event_data.created_on <= range.to)
    && filter.printers.as_ref().map_or(true, |ids| ids.contains(&event_data.printer_id))
    && filter.time_slot_id.as_ref().map_or(true, |id| event_data.time_slot_id == *id)
    && filter.no_products.map_or(true, |range| event_data.no_products >= range.from && event_data.no_products <= range.to)
}


---

Would you like me to turn this into a PDF with diagram or share the Rust code base structure for the server?

