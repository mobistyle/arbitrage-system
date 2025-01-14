#!/bin/bash

# Проверяем конфигурацию
cargo run --bin check_config || {
    echo "Configuration check failed. Please check the errors above."
    exit 1
}

# Запускаем основную программу
echo "Starting arbitrage system..."
cargo run --release
