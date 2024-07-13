# Используем официальный образ Rust
FROM rust:1.60 as builder

# Устанавливаем рабочую директорию
WORKDIR /usr/src/my_rust_library

# Копируем файлы проекта
COPY . .

# Сборка библиотеки
RUN cargo build --release

# Создаем новый минималистичный образ для результата
FROM debian:buster-slim

# Копируем скомпилированную библиотеку из предыдущего шага
COPY --from=builder /usr/src/my_rust_library/target/release/libmy_rust_library.so /usr/local/lib/libmy_rust_library.so

# Устанавливаем рабочую директорию
WORKDIR /usr/src/my_rust_library

# Копируем исходные файлы (для тестов и примеров)
COPY . .

# Устанавливаем необходимые зависимости
RUN apt-get update && apt-get install -y gcc

# Устанавливаем переменные окружения для динамического линкования
ENV LD_LIBRARY_PATH=/usr/local/lib

# Команда по умолчанию
CMD ["bash"]
