#!/bin/bash

# Создаем файл .env если его нет
if [ ! -f .env ]; then
    cp .env.example .env
    echo "Created .env file from .env.example"
fi

# Обновляем зависимости и собираем проект
cargo update
cargo build --release

echo "Setup completed!"
