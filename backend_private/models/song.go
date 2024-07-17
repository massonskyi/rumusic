package models

import "github.com/jinzhu/gorm"

type Song struct {
	gorm.Model
	Title  string `json:"title"`
	Artist string `json:"artist"`
	URL    string `json:"url"`
	UserID uint   `json:"user_id"`
}
