#!/bin/bash

set -e

# Load environment variables from .env.development
if [ -f .env.development ]; then
    export $(cat .env.development | grep -v '^#' | xargs)
fi

# Override for testing
export DATABASE_URL=postgresql://postgres:password@localhost:5432/blog
export SQLX_OFFLINE=false
export APP_DATABASE__HOST=localhost

# Run the tests
cargo test "$@"
