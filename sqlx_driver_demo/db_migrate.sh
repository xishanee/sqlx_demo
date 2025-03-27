#!/bin/bash

# Create database
# "postgresql://{postgres_user}:{postgres_password}@{postgres_host}:{postgres_port}/{postgres_db}"
export DATABASE_URL=postgresql://postgres:123456@localhost:5432/sqlx_demo_db

sqlx database create
#sqlx migrate add db_init
#sqlx migrate add add_config_table
sqlx migrate run