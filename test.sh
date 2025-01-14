#!/bin/bash

echo "=== Arbitrage System Test Suite ==="
echo "Starting tests at $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
echo "User: mobistyle"

# Проверяем наличие необходимых инструментов
command -v cargo >/dev/null 2>&1 || { echo "✗ Cargo is required but not installed. Aborting."; exit 1; }

# Компилируем проект
echo -n "Building project... "
if cargo build --quiet; then
    echo "✓ Success"
else
    echo "✗ Build failed!"
    exit 1
fi

# Запускаем тесты
echo -e "\nRunning tests..."
RUST_BACKTRACE=1 cargo test -- --nocapture

if [ $? -eq 0 ]; then
    echo -e "\n✓ All tests completed successfully!"
else
    echo -e "\n✗ Tests failed!"
    exit 1
fi

# Проверяем конфигурацию
echo -e "\nChecking configuration..."
cargo run --quiet --bin check_config

echo -e "\nTest suite completed at $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
