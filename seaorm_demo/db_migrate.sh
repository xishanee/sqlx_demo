#!/bin/bash

# Define database connection details
DATABASE_URL=postgresql://postgres:123456@localhost:5432/seaorm_demo

# Check if sea-orm-cli is installed
if ! command -v sea-orm-cli; then
    echo "sea-orm-cli is not installed. exit..."
    exit 1
fi

# Run migrations
echo "Running database migrations..."
#sea-orm-cli migrate init --database-url "$DATABASE_URL" --database-schema kres
sea-orm-cli migrate refresh --database-url "$DATABASE_URL" --database-schema public

echo "Database migrations completed successfully."