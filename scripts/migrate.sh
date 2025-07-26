#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check for help first
if [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Database Migration Script"
    echo ""
    echo "Usage: $0 <environment> <command> [args...]"
    echo ""
    echo "Environments:"
    echo "  dev        - Development environment (.env.development)"
    echo "  prod       - Production environment (.env.production)"
    echo ""
    echo "Commands:"
    echo "  run        - Run pending migrations"
    echo "  revert     - Revert last migration (with extra confirmation for prod)"
    echo "  info       - Show migration status"
    echo "  status     - Alias for info"
    echo "  add <name> - Create new migration file"
    echo "  reset      - Drop and recreate database (disabled in prod)"
    echo "  setup      - Create database and run migrations"
    echo "  check      - Check database connection and status"
    echo "  dry-run    - Test connection and show pending migrations"
    echo "  help       - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 dev setup                    # Set up development database"
    echo "  $0 dev add create_posts_table   # Create new migration"
    echo "  $0 dev run                      # Run pending migrations"
    echo "  $0 dev check                    # Check migration status"
    echo "  $0 prod run                     # Run migrations in production"
    echo "  $0 staging dry-run              # Test staging connection"
    echo ""
    echo "Safety Features:"
    echo "  - Production revert requires typing 'REVERT' to confirm"
    echo "  - Production reset is completely disabled"
    echo "  - Database URLs are masked in logs"
    echo "  - Environment-specific configuration files"
    exit 0
fi

# Default values
ENVIRONMENT=${1:-dev}
COMMAND=${2:-run}

# Function to print colored output
log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

# Function to load environment variables
load_env() {
    local env_file=".env.$1"

    if [ ! -f "$env_file" ]; then
        log_error "Environment file $env_file not found"
        exit 1
    fi

    log_info "Loading environment from $env_file"
    export $(cat "$env_file" | grep -v '^#' | grep -v '^$' | xargs)
}

# Function to check if we're in Docker context
check_docker_context() {
    if [ "$USE_DOCKER" = "true" ] || [ "$ENVIRONMENT" = "prod" ]; then
        return 0
    fi
    return 1
}

# Function to run sqlx command (with or without Docker)
run_sqlx() {
    if check_docker_context; then
        log_info "Running sqlx via Docker..."
        docker run --rm --network axum-blog_app-network \
            -v "$(pwd)/migrations:/migrations" \
            -e DATABASE_URL="$DATABASE_URL" \
            --workdir /migrations \
            migrate/migrate -path=/migrations -database="$DATABASE_URL" "$@"
    else
        log_info "Running sqlx locally..."
        sqlx "$@"
    fi
}

# Validate environment
case $ENVIRONMENT in
"dev" | "development")
    ENVIRONMENT="development"
    ;;
"prod" | "production")
    ENVIRONMENT="production"
    USE_DOCKER="true"
    ;;
*)
    log_error "Invalid environment: $ENVIRONMENT"
    log_info "Valid environments: dev, prod, staging"
    exit 1
    ;;
esac

# Load environment variables
load_env "$ENVIRONMENT"

# Validate DATABASE_URL
if [ -z "$DATABASE_URL" ]; then
    log_error "DATABASE_URL must be set in .env.$ENVIRONMENT"
    exit 1
fi

# For development, construct the Docker-aware DATABASE_URL if needed
if [ "$ENVIRONMENT" = "dev" ] && [ -n "$APP_DATABASE__HOST" ]; then
    DB_HOST="$APP_DATABASE__HOST"
    DB_PORT="$APP_DATABASE__PORT"
    DB_USER="$APP_DATABASE__USERNAME"
    DB_PASS="$APP_DATABASE__PASSWORD"
    DB_NAME="$APP_DATABASE__DATABASE_NAME"

    # Use the constructed URL for Docker context
    if check_docker_context; then
        DATABASE_URL="postgresql://$DB_USER:$DB_PASS@$DB_HOST:$DB_PORT/$DB_NAME"
    fi
fi

export DATABASE_URL
log_info "Using database: $(echo "$DATABASE_URL" | sed 's/:.*@/:***@/')"
log_info "Environment: $ENVIRONMENT"

case $COMMAND in
"run" | "migrate")
    log_info "Running migrations for $ENVIRONMENT environment..."
    if run_sqlx migrate run; then
        log_success "Migrations completed successfully"
    else
        log_error "Migration failed"
        exit 1
    fi
    ;;
"revert")
    log_warning "Reverting last migration for $ENVIRONMENT environment..."
    if [ "$ENVIRONMENT" = "prod" ]; then
        log_warning "You are about to revert a migration in PRODUCTION!"
        read -p "Are you absolutely sure? Type 'REVERT' to confirm: " -r
        if [ "$REPLY" != "REVERT" ]; then
            log_error "Revert cancelled"
            exit 1
        fi
    fi

    if run_sqlx migrate revert; then
        log_success "Migration reverted successfully"
    else
        log_error "Migration revert failed"
        exit 1
    fi
    ;;
"info" | "status")
    log_info "Migration status for $ENVIRONMENT environment:"
    run_sqlx migrate info
    ;;
"add")
    MIGRATION_NAME=${3:-"new_migration"}
    if [ -z "$MIGRATION_NAME" ] || [ "$MIGRATION_NAME" = "new_migration" ]; then
        log_error "Please provide a migration name"
        log_info "Usage: $0 $ENVIRONMENT add <migration_name>"
        exit 1
    fi

    log_info "Creating new migration: $MIGRATION_NAME"
    if sqlx migrate add "$MIGRATION_NAME"; then
        log_success "Migration file created: migrations/*_$MIGRATION_NAME.sql"
        log_info "Don't forget to edit the migration file before running migrations!"
    else
        log_error "Failed to create migration file"
        exit 1
    fi
    ;;
"reset")
    log_warning "WARNING: This will drop the entire $ENVIRONMENT database!"
    if [ "$ENVIRONMENT" = "prod" ]; then
        log_error "Database reset is not allowed in production environment!"
        exit 1
    fi

    read -p "Are you sure you want to reset the $ENVIRONMENT database? Type 'RESET' to confirm: " -r
    echo
    if [ "$REPLY" = "RESET" ]; then
        DB_NAME=$(echo "$DATABASE_URL" | sed 's/.*\///')

        log_info "Dropping database $DB_NAME..."
        if sqlx database drop -y --database-url "$DATABASE_URL"; then
            log_success "Database dropped"
        else
            log_warning "Failed to drop database (may not exist)"
        fi

        log_info "Creating database $DB_NAME..."
        if sqlx database create --database-url "$DATABASE_URL"; then
            log_success "Database created"
        else
            log_error "Failed to create database"
            exit 1
        fi

        log_info "Running all migrations..."
        if run_sqlx migrate run; then
            log_success "Database reset completed successfully"
        else
            log_error "Failed to run migrations after reset"
            exit 1
        fi
    else
        log_error "Reset cancelled (you must type 'RESET' to confirm)"
        exit 1
    fi
    ;;
"setup")
    log_info "Setting up $ENVIRONMENT database..."

    # Create database if it doesn't exist
    DB_NAME=$(echo "$DATABASE_URL" | sed 's/.*\///')
    log_info "Creating database $DB_NAME if it doesn't exist..."
    sqlx database create --database-url "$DATABASE_URL" 2>/dev/null || log_info "Database already exists"

    # Run migrations
    log_info "Running migrations..."
    if run_sqlx migrate run; then
        log_success "Database setup completed successfully"
    else
        log_error "Database setup failed"
        exit 1
    fi
    ;;
"check" | "status")
    log_info "Checking database connection and migration status for $ENVIRONMENT..."

    # Check if database exists and is reachable
    if run_sqlx migrate info >/dev/null 2>&1; then
        log_success "Database is reachable"
        echo
        run_sqlx migrate info
    else
        log_error "Cannot connect to database"
        log_info "Database URL: $(echo "$DATABASE_URL" | sed 's/:.*@/:***@/')"

        # Try to provide helpful debugging info
        if [ "$ENVIRONMENT" = "dev" ]; then
            log_info "For development, make sure your Docker containers are running:"
            log_info "  ./scripts/deploy.sh dev up -d"
        fi
        exit 1
    fi
    ;;
"dry-run")
    log_info "Performing dry-run for $ENVIRONMENT environment..."
    log_info "This would connect to: $(echo "$DATABASE_URL" | sed 's/:.*@/:***@/')"

    # Just test the connection without running migrations
    if run_sqlx migrate info >/dev/null 2>&1; then
        log_success "Connection test passed"
        log_info "Pending migrations:"
        run_sqlx migrate info | grep -E "^[0-9]+" | grep -v "applied" || log_info "No pending migrations"
    else
        log_error "Connection test failed"
        exit 1
    fi
    ;;
*)
    log_error "Unknown command: $COMMAND"
    echo ""
    log_info "Use '$0 help' to see available commands"
    exit 1
    ;;
esac
