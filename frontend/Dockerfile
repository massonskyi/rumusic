# Указываем базовый образ
FROM node:14-alpine

# Устанавливаем рабочую директорию в контейнере
WORKDIR /rumusic/frontend/

# Copy package.json and package-lock.json to the working directory
COPY package*.json ./

# Install dependencies
RUN npm install

# Install serve if not present
RUN npm install -g serve

# Copy the remaining application code to the working directory
COPY . .

# Build your React app
RUN npm run build

# Expose port 5000 to allow outside access to the app
EXPOSE 5000

# Command to run the app
CMD ["serve", "-s", "build"]
