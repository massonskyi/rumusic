package db

import (
	"backend_engine/config"
	"backend_engine/models"
	"fmt"
	"log"

	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/postgres"
)

var DB *gorm.DB

var cfg *config.Config = config.LoadConfig()

func Connect() {
	// connect to database with postgres database connection string and default port number
	var err error

	connStr := fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=%s sslmode=disable",
		cfg.DbHost,
		cfg.DbUser,
		cfg.DbPass,
		cfg.DbName,
		cfg.DbPort,
	)

	DB, err = gorm.Open("postgres", connStr)

	if err != nil {
		log.Fatal("Failed to connect to database: ", err)
	}

	fmt.Println("Database connected successfully")

	DB.AutoMigrate(&models.User{}, &models.Song{})
}
