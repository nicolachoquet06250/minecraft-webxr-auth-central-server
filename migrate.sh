#!/bin/bash

echo "=== Database Migration Tool ==="
echo ""

if [ -z "$1" ]; then
    echo "Usage: migrate.sh [command]"
    echo ""
    echo "Commands:"
    echo "  up       - Apply all pending migrations"
    echo "  down     - Rollback the last migration"
    echo "  fresh    - Drop all tables and re-run all migrations"
    echo "  refresh  - Rollback all and re-run all migrations"
    echo "  reset    - Rollback all migrations"
    echo "  status   - Check migration status"
    echo "  generate [name] - Generate a new migration file"
    echo ""
    exit 1
fi

cd backend/migration

COMMAND=$1
MIGRATION_NAME=$2

case $COMMAND in
    up)
        echo "Applying pending migrations..."
        cargo run -- up
        ;;
    down)
        echo "Rolling back last migration..."
        cargo run -- down
        ;;
    fresh)
        echo "Dropping all tables and re-running migrations..."
        cargo run -- fresh
        ;;
    refresh)
        echo "Refreshing database..."
        cargo run -- refresh
        ;;
    reset)
        echo "Resetting database..."
        cargo run -- reset
        ;;
    status)
        echo "Checking migration status..."
        cargo run -- status
        ;;
    generate)
        if [ -z "$MIGRATION_NAME" ]; then
            echo "Error: Migration name is required"
            echo "Usage: migrate.sh generate [migration_name]"
            exit 1
        fi
        echo "Generating migration: $MIGRATION_NAME"
        cargo run -- generate $MIGRATION_NAME
        ;;
    *)
        echo "Unknown command: $COMMAND"
        exit 1
        ;;
esac

cd ../..
