# Используем официальный образ Rust
FROM rust:latest AS builder

# Устанавливаем необходимые зависимости
RUN apt-get update && apt-get install -y libpq-dev

# Устанавливаем Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Устанавливаем рабочую директорию
WORKDIR /usr/src/app

# Копируем файлы проекта
COPY . .

# Собираем проект
RUN cargo build --release

# Запускаем второй этап для уменьшения размера образа
FROM ubuntu:latest

# Устанавливаем необходимые библиотеки
RUN apt-get update && apt-get install -y libpq-dev netcat-openbsd

# Копируем бинарный файл из первого этапа
COPY --from=builder /usr/src/app/target/release/backend_main /usr/local/bin/backend_main

# Копируем Diesel CLI из предыдущего шага
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/

## Копируем файл .env
#COPY --from=builder /usr/src/app/.env /usr/src/app/.env

# Копируем файлы проекта для выполнения миграций
COPY --from=builder /usr/src/app /usr/src/app

# Устанавливаем рабочую директорию
WORKDIR /usr/src/app

# Запуск приложения
CMD ["sh", "-c", "export HOST=$(hostname -i) && backend_main"]