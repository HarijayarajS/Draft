#!/bin/sh
# Wrapper around sqlx db reset that ensures database is created with C collation

DB_NAME="mydb"   # replace with your database name
DB_USER="postgres"   # replace with your postgres user
DB_HOST="localhost"  # replace if remote
DB_PORT="5432"       # replace if non-default

DATABASE_URL="postgres://${DB_USER}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

echo "Dropping old database: $DB_NAME"
sqlx database drop --database-url "$DATABASE_URL" -y

echo "Recreating database with C collation..."
createdb \
  --locale=C \
  --encoding=UTF8 \
  --template=template0 \
  -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" \
  "$DB_NAME"

echo "Running migrations..."
sqlx migrate run --database-url "$DATABASE_URL"

echo "✅ $DB_NAME has been reset with C collation."



#!/bin/sh
# Script to initialize PostgreSQL database + user with C collation

DB_NAME="spont"
DB_USER="spont"
DB_PASS="spont2025"
DB_HOST="localhost"
DB_PORT="5432"

echo "🔹 Creating user and database: $DB_NAME"

# Create user (ignore error if it already exists)
psql -h "$DB_HOST" -p "$DB_PORT" -U postgres -d postgres -v ON_ERROR_STOP=1 <<EOF
DO
\$do\$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = '$DB_USER') THEN
      CREATE USER $DB_USER WITH ENCRYPTED PASSWORD '$DB_PASS';
   END IF;
END
\$do\$;
EOF

# Drop DB if it exists (for clean setup)
psql -h "$DB_HOST" -p "$DB_PORT" -U postgres -d postgres -c "DROP DATABASE IF EXISTS $DB_NAME;"

# Create DB with C collation
createdb \
  --locale=C \
  --encoding=UTF8 \
  --template=template0 \
  -h "$DB_HOST" -p "$DB_PORT" -U postgres \
  "$DB_NAME"

# Set ownership and privileges
psql -h "$DB_HOST" -p "$DB_PORT" -U postgres -d postgres <<EOF
ALTER DATABASE $DB_NAME OWNER TO $DB_USER;
GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;
GRANT USAGE, CREATE ON SCHEMA public TO $DB_USER;
ALTER DATABASE $DB_NAME SET TIMEZONE TO 'Asia/Calcutta';
ALTER USER $DB_USER CREATEDB CREATEROLE LOGIN;
EOF

echo "✅ Database '$DB_NAME' initialized with user '$DB_USER' (collation = C)."
