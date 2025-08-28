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