package config

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

type Config struct {
	DbHost                 string
	DbPort                 string
	DbName                 string
	DbUser                 string
	DbPass                 string
	BackendSecretCookieKey string
}

func LoadConfig() *Config {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	config := &Config{
		DbHost:                 os.Getenv("DB_HOST"),
		DbPort:                 os.Getenv("DB_PORT"),
		DbName:                 os.Getenv("DB_NAME"),
		DbUser:                 os.Getenv("DB_USER"),
		DbPass:                 os.Getenv("DB_PASS"),
		BackendSecretCookieKey: os.Getenv("BACKEND_SECRET_COOKIE_KEY"),
	}

	return config
}
