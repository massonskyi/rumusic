# Указываем базовый образ
FROM node:16

# Устанавливаем рабочую директорию в контейнере
WORKDIR /app

# Копируем package.json и yarn.lock в контейнер
COPY package.json yarn.lock ./

# Устанавливаем зависимости
RUN yarn install

# Копируем весь проект в контейнер
COPY . .

# Создаем сборку проекта
RUN yarn build

# Открываем порт для сервера
EXPOSE 3000

# Устанавливаем сервер для обслуживания статических файлов
RUN yarn global add serve

# Команда для запуска сервера
CMD ["serve", "-s", "build"]
