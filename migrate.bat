@echo off
echo === Database Migration Tool ===
echo.

if "%1"=="" (
    echo Usage: migrate.bat [command]
    echo.
    echo Commands:
    echo   up       - Apply all pending migrations
    echo   down     - Rollback the last migration
    echo   fresh    - Drop all tables and re-run all migrations
    echo   refresh  - Rollback all and re-run all migrations
    echo   reset    - Rollback all migrations
    echo   status   - Check migration status
    echo   generate [name] - Generate a new migration file
    echo.
    exit /b 1
)

cd backend\migration

set COMMAND=%1
set MIGRATION_NAME=%2

if "%COMMAND%"=="up" (
    echo Applying pending migrations...
    cargo run -- up
) else if "%COMMAND%"=="down" (
    echo Rolling back last migration...
    cargo run -- down
) else if "%COMMAND%"=="fresh" (
    echo Dropping all tables and re-running migrations...
    cargo run -- fresh
) else if "%COMMAND%"=="refresh" (
    echo Refreshing database...
    cargo run -- refresh
) else if "%COMMAND%"=="reset" (
    echo Resetting database...
    cargo run -- reset
) else if "%COMMAND%"=="status" (
    echo Checking migration status...
    cargo run -- status
) else if "%COMMAND%"=="generate" (
    if "%MIGRATION_NAME%"=="" (
        echo Error: Migration name is required
        echo Usage: migrate.bat generate [migration_name]
        exit /b 1
    )
    echo Generating migration: %MIGRATION_NAME%
    cargo run -- generate %MIGRATION_NAME%
) else (
    echo Unknown command: %COMMAND%
    exit /b 1
)

cd ..\..
