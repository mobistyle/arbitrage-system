#!/bin/bash

# Функция для корректного завершения
cleanup() {
    echo "Shutting down..."
    if [ -f "pid.txt" ]; then
        kill $(cat pid.txt) 2>/dev/null
        rm pid.txt
    fi
    exit 0
}

# Устанавливаем обработчик SIGTERM и SIGINT
trap cleanup SIGTERM SIGINT

echo "Starting Arbitrage Monitor..."

# Проверяем существование директории для логов
mkdir -p logs

# Компилируем проект
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

# Запускаем программу
while true; do
    echo "Running Arbitrage Monitor..."
    ./target/release/arbitrage-system &
    echo $! > pid.txt
    
    wait $!
    
    if [ $? -eq 0 ]; then
        echo "Program finished successfully"
        break
    else
        echo "Program crashed, restarting in 5 seconds..."
        sleep 5
    fi
done