package services

import (
	"backend_engine/models"
	"backend_engine/utils"

	"github.com/jinzhu/gorm"
)

func RegisterUser(db *gorm.DB, username, password string) error {
	hashedPassword, err := utils.HashPassword(password)
	if err != nil {
		return err
	}
	user := models.User{Username: username, HashedPassword: hashedPassword}
	return db.Create(&user).Error
}

func AuthenticateUser(db *gorm.DB, username, password string) (bool, error) {
	var user models.User
	if err := db.Where("username = ?", username).First(&user).Error; err != nil {
		return false, err
	}
	return utils.CheckPasswordHash(password, user.HashedPassword), nil
}
