package models

type Track struct {
	ID     uint   `gorm:"primary_key"`
	Title  string `gorm:"not null"`
	Artist string `gorm:"not null"`
	URL    string `gorm:"not null"`
	UserID uint
}
