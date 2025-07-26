#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }
log_header() { echo -e "${MAGENTA}🚀 $1${NC}"; }
log_step() { echo -e "${CYAN}📋 $1${NC}"; }

# Check for help first
if [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Docker Deployment Script"
    echo ""
    echo "Usage: $0 <environment> <command> [args...]"
    echo ""
    echo "Environments:"
    echo "  dev        - Development environment (.env.dev)"
    echo "  staging    - Staging environment (.env.staging)"
    echo "  prod       - Production environment (.env.prod)"
    echo "  secrets    - Manage Docker secrets"
    echo ""
    echo "Development Commands:"
    echo "  up         - Start services (use -d for detached)"
    echo "  down       - Stop and remove services"
    echo "  restart    - Restart services"
    echo "  logs       - View logs (-f to follow)"
    echo "  build      - Rebuild images"
    echo "  ps         - Show running containers"
    echo "  shell      - Open shell in app container"
    echo "  clean      - Clean up unused Docker resources"
    echo "  health     - Check service health"
    echo ""
    echo "Production Commands:"
    echo "  deploy     - Deploy to Docker Swarm"
    echo "  remove     - Remove from swarm"
    echo "  update     - Update services in swarm"
    echo "  rollback   - Rollback to previous version"
    echo "  logs       - View service logs"
    echo "  status     - Check service status"
    echo "  scale      - Scale services"
    echo ""
    echo "Secret Commands:"
    echo "  create     - Create all secrets from files"
    echo "  list       - List all secrets"
    echo "  remove     - Remove a specific secret"
    echo "  generate   - Generate secret template files"
    echo ""
    echo "Examples:"
    echo "  $0 dev up -d              # Start dev environment detached"
    echo "  $0 dev logs -f app        # Follow app logs"
    echo "  $0 staging deploy         # Deploy to staging"
    echo "  $0 prod deploy            # Deploy to production"
    echo "  $0 prod scale app=3       # Scale app service to 3 replicas"
    echo "  $0 secrets create         # Create all production secrets"
    echo ""
    echo "Health Checks:"
    echo "  - Automatic health checks for all environments"
    echo "  - Service dependency validation"
    echo "  - Database connectivity testing"
    echo "  - Rollback on deployment failure"
    exit 0
fi

ENVIRONMENT=${1:-dev}
COMMAND=${2:-up}

# Function to load and validate environment
load_environment() {
    local env_file=".env.$1"

    if [ ! -f "$env_file" ]; then
        log_error "Environment file $env_file not found"
        exit 1
    fi

    log_info "Loading environment from $env_file"
    set -a # automatically export all variables
    source "$env_file"
    set +a

    # Validate required variables
    if [ -z "$APP_PORT" ]; then
        log_warning "APP_PORT not set, using default 3000"
        export APP_PORT=3000
    fi
}

# Function to check if Docker is running
check_docker() {
    if ! docker info >/dev/null 2>&1; then
        log_error "Docker is not running. Please start Docker first."
        exit 1
    fi
}

# Function to wait for service health
wait_for_health() {
    local service_name=$1
    local max_attempts=${2:-30}
    local attempt=1

    log_info "Waiting for $service_name to be healthy..."

    while [ $attempt -le "$max_attempts" ]; do
        if docker-compose ps | grep -q "$service_name.*healthy\|$service_name.*Up"; then
            log_success "$service_name is healthy"
            return 0
        fi

        if [ $attempt -eq 1 ]; then
            echo -n "  Attempt: "
        fi
        echo -n "$attempt "

        sleep 2
        attempt=$((attempt + 1))
    done

    echo
    log_error "$service_name failed to become healthy within $((max_attempts * 2)) seconds"
    return 1
}

# Function to check service connectivity
check_connectivity() {
    local environment=$1

    log_step "Checking service connectivity..."

    # Check database connectivity
    if [ "$environment" != "prod" ]; then
        if docker-compose exec -T db pg_isready -U "${POSTGRES_USER:-postgres}" >/dev/null 2>&1; then
            log_success "Database is accessible"
        else
            log_warning "Database connectivity check failed"
        fi
    fi

    # Check app health endpoint if available
    if [ -n "$APP_PORT" ]; then
        local health_url="http://localhost:${APP_PORT}/health_check"
        if command -v curl >/dev/null 2>&1; then
            if curl -sf "$health_url" >/dev/null 2>&1; then
                log_success "Application health endpoint is responding"
            else
                log_info "Health endpoint not available or not implemented"
            fi
        fi
    fi
}

# Validate environment and check Docker
check_docker

case $ENVIRONMENT in
"dev" | "development")
    ENVIRONMENT="dev"
    log_header "Starting development environment..."
    load_environment "dev"

    case $COMMAND in
    "up")
        log_step "Starting services..."
        docker-compose -f docker-compose.yml -f docker-compose.dev.yml up "${@:3}"
        if [[ "${@:3}" == *"-d"* ]]; then
            log_step "Waiting for services to be ready..."
            sleep 5
            wait_for_health "db" 15
            wait_for_health "app" 30
            check_connectivity "dev"
            log_success "Development environment is running!"
            log_info "Access your application at: http://localhost:${APP_PORT}"
        fi
        ;;
    "down")
        log_step "Stopping services..."
        docker-compose -f docker-compose.yml -f docker-compose.dev.yml down "${@:3}"
        log_success "Development environment stopped"
        ;;
    "restart")
        log_step "Restarting services..."
        docker-compose -f docker-compose.yml -f docker-compose.dev.yml restart "${@:3}"
        log_success "Services restarted"
        ;;
    "logs")
        docker-compose -f docker-compose.yml -f docker-compose.dev.yml logs "${@:3}"
        ;;
    "build")
        log_step "Building images..."
        docker-compose -f docker-compose.yml -f docker-compose.dev.yml build "${@:3}"
        log_success "Images built successfully"
        ;;
    "ps")
        docker-compose -f docker-compose.yml -f docker-compose.dev.yml ps
        ;;
    "shell")
        SERVICE=${3:-app}
        log_info "Opening shell in $SERVICE container..."
        docker-compose -f docker-compose.yml -f docker-compose.dev.yml exec "$SERVICE" /bin/bash
        ;;
    "clean")
        log_step "Cleaning up unused Docker resources..."
        docker system prune -f
        docker volume prune -f
        log_success "Cleanup completed"
        ;;
    "health")
        check_connectivity "dev"
        ;;
    *)
        log_error "Unknown development command: $COMMAND"
        log_info "Use '$0 help' to see available commands"
        exit 1
        ;;
    esac
    ;;

"prod" | "production")
    ENVIRONMENT="prod"
    log_header "Deploying to production environment..."
    load_environment "prod"

    # Check if Docker Swarm is initialized
    if ! docker info | grep -q "Swarm: active"; then
        log_step "Initializing Docker Swarm..."
        docker swarm init
        log_success "Docker Swarm initialized"
    fi

    # Check if secrets exist
    REQUIRED_SECRETS=("postgres_password" "postgres_db" "postgres_user" "database_url" "jwt_secret")
    log_step "Validating required secrets..."
    for secret in "${REQUIRED_SECRETS[@]}"; do
        if ! docker secret inspect "$secret" >/dev/null 2>&1; then
            log_error "Secret '$secret' not found. Please create it first:"
            log_info "   docker secret create $secret /path/to/secret/file"
            log_info "   Or use: $0 secrets generate"
            exit 1
        fi
    done
    log_success "All required secrets are available"

    case $COMMAND in
    "up" | "deploy")
        log_step "Deploying to Docker Swarm..."
        docker stack deploy -c docker-compose.yml -c docker-compose.prod.yml axum-blog
        log_success "Production deployment initiated"
        log_info "Check status with: $0 prod status"
        log_info "View logs with:   $0 prod logs"
        ;;
    "down" | "remove")
        log_step "Removing production stack..."
        docker stack rm axum-blog
        log_success "Production stack removal initiated"
        ;;
    "update")
        log_step "Updating production services..."
        docker stack deploy -c docker-compose.yml -c docker-compose.prod.yml axum-blog
        log_success "Production services updated"
        ;;
    "rollback")
        SERVICE=${3:-axum-blog_app}
        log_warning "Rolling back service: $SERVICE"
        docker service rollback "$SERVICE"
        log_success "Rollback completed for $SERVICE"
        ;;
    "logs")
        SERVICE=${3:-axum-blog_app}
        FOLLOW=${4:--f}
        log_info "Showing logs for $SERVICE ($FOLLOW)"
        docker service logs "$FOLLOW" "$SERVICE"
        ;;
    "status")
        log_info "Production stack status:"
        docker stack services axum-blog
        echo
        log_info "Service details:"
        docker stack ps axum-blog
        ;;
    "scale")
        if [ -z "$3" ]; then
            log_error "Please specify scaling parameters (e.g., app=3)"
            exit 1
        fi
        log_step "Scaling services: $3"
        docker service scale "axum-blog_$3"
        log_success "Scaling completed"
        ;;
    *)
        log_error "Unknown production command: $COMMAND"
        log_info "Available commands: deploy, remove, update, rollback, logs, status, scale"
        exit 1
        ;;
    esac
    ;;

"secrets")
    log_header "Managing Docker secrets..."

    # Ensure we're in swarm mode for secrets
    if ! docker info | grep -q "Swarm: active"; then
        log_step "Initializing Docker Swarm for secrets management..."
        docker swarm init
        log_success "Docker Swarm initialized"
    fi

    case $COMMAND in
    "create")
        log_step "Creating production secrets from files..."

        # Create secrets directory if it doesn't exist
        if [ ! -d "secrets" ]; then
            log_info "Creating secrets directory..."
            mkdir -p secrets
        fi

        SECRETS=("postgres_password" "postgres_db" "postgres_user" "database_url" "jwt_secret")

        for secret in "${SECRETS[@]}"; do
            secret_file="secrets/$secret"

            if [ ! -f "$secret_file" ]; then
                log_warning "Secret file $secret_file not found, skipping..."
                continue
            fi

            # Remove existing secret if it exists
            if docker secret inspect "$secret" >/dev/null 2>&1; then
                log_warning "Secret $secret already exists, removing first..."
                docker secret rm "$secret" || true
            fi

            # Create the secret
            if docker secret create "$secret" "$secret_file"; then
                log_success "Created secret: $secret"
            else
                log_error "Failed to create secret: $secret"
            fi
        done

        log_success "Secret creation process completed"
        log_info "Use '$0 secrets list' to verify"
        ;;
    "generate")
        log_step "Generating secret template files..."

        mkdir -p secrets

        # Generate template files with instructions
        cat >secrets/postgres_password <<'EOF'
# Replace this with your actual PostgreSQL password
# This file should contain ONLY the password, no newlines
your_secure_database_password_here
EOF

        cat >secrets/postgres_db <<'EOF'
blog
EOF

        cat >secrets/postgres_user <<'EOF'
postgres
EOF

        cat >secrets/jwt_secret <<'EOF'
# Replace this with a secure JWT secret (at least 32 characters)
# You can generate one with: openssl rand -base64 32
your_jwt_secret_key_here_minimum_32_characters
EOF

        # Generate database URL template
        cat >secrets/database_url <<'EOF'
# Replace with your actual database connection string
# Format: postgresql://username:password@host:port/database
postgresql://postgres:your_secure_database_password_here@db:5432/blog
EOF

        log_success "Secret template files generated in secrets/ directory"
        log_warning "IMPORTANT: Edit these files with your actual secrets before running 'create'"
        log_info "Files created:"
        ls -la secrets/
        ;;
    "list")
        log_info "Current Docker secrets:"
        docker secret ls
        ;;
    "remove")
        SECRET_NAME=${3:-""}
        if [ -z "$SECRET_NAME" ]; then
            log_error "Please specify a secret name to remove"
            log_info "Usage: $0 secrets remove <secret_name>"
            log_info "Available secrets:"
            docker secret ls --format "table {{.Name}}\t{{.CreatedAt}}"
            exit 1
        fi

        if docker secret inspect "$SECRET_NAME" >/dev/null 2>&1; then
            log_warning "Removing secret: $SECRET_NAME"
            if docker secret rm "$SECRET_NAME"; then
                log_success "Secret $SECRET_NAME removed successfully"
            else
                log_error "Failed to remove secret $SECRET_NAME"
                exit 1
            fi
        else
            log_error "Secret $SECRET_NAME not found"
            exit 1
        fi
        ;;
    "validate")
        log_step "Validating production secrets..."
        REQUIRED_SECRETS=("postgres_password" "postgres_db" "postgres_user" "database_url" "jwt_secret")
        all_valid=true

        for secret in "${REQUIRED_SECRETS[@]}"; do
            if docker secret inspect "$secret" >/dev/null 2>&1; then
                log_success "✓ $secret"
            else
                log_error "✗ $secret (missing)"
                all_valid=false
            fi
        done

        if [ "$all_valid" = true ]; then
            log_success "All required secrets are available"
        else
            log_error "Some secrets are missing. Use '$0 secrets generate' then '$0 secrets create'"
            exit 1
        fi
        ;;
    *)
        log_error "Unknown secrets command: $COMMAND"
        log_info "Available commands: create, generate, list, remove, validate"
        exit 1
        ;;
    esac
    ;;

*)
    echo "Usage: $0 <environment> <command> [args...]"
    echo ""
    echo "Environments:"
    echo "  dev        - Development with .env file"
    echo "  prod       - Production with Docker secrets"
    echo "  secrets    - Manage Docker secrets"
    echo ""
    echo "Development commands:"
    echo "  up         - Start services"
    echo "  down       - Stop services"
    echo "  logs       - View logs"
    echo "  build      - Rebuild images"
    echo ""
    echo "Production commands:"
    echo "  deploy     - Deploy to swarm"
    echo "  remove     - Remove from swarm"
    echo "  logs       - View service logs"
    echo "  status     - Check service status"
    echo ""
    echo "Examples:"
    echo "  $0 dev up -d"
    echo "  $0 prod deploy"
    echo "  $0 secrets create"
    exit 1
    ;;
esac
