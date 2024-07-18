package models

type Role struct {
	ID          uint `gorm:"primary_key"`
	Name        string
	Permissions map[string]interface{} `gorm:"type:json"`
}
