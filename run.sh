#!/bin/bash

# Load environment variables from .env file if it exists
if [ -f .env ]; then
    echo "Loading environment variables from .env file..."
    export $(cat .env | grep -v '^#' | xargs)
fi

echo "Starting arbitrage system at $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
echo "User: $(whoami)"

# Set config path
export CONFIG_PATH="config/basic_config.yaml"

# Run the application
cargo run --release
