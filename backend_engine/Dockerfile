# Указываем базовый образ
FROM python:3.12-slim

# Устанавливаем рабочую директорию в контейнере
WORKDIR /rumusic/backend/

# Копируем файлы requirements.txt в контейнер
COPY requirements.txt .

# Устанавливаем зависимости
RUN pip install --no-cache-dir -r requirements.txt

# Копируем весь проект в контейнер
COPY . .

ENV DB_HOST=db
ENV DB_PORT=5432
ENV DB_USER=user
ENV DB_PASS=1111
ENV DB_NAME=musicdb
ENV BACKEND_SECRET_COOKIE_KEY=5

# Копируем файл конфигурации логгера
COPY logging_config.json /rumusic/backend/

# Открываем порт для сервера
EXPOSE 8000

# Команда для запуска приложения с указанием файла конфигурации логгера
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000", "--log-config", "/rumusic/backend/logging_config.json"]