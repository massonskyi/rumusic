package models

import (
	"time"
)

type User struct {
	ID             uint      `gorm:"primary_key"`
	Name           string    `gorm:"type:varchar(50);not null"`
	Email          string    `gorm:"type:varchar(320);unique_index;not null"`
	Username       string    `gorm:"type:varchar(50);not null"`
	RegisteredAt   time.Time `gorm:"default:current_timestamp"`
	RoleID         uint
	Role           Role
	Storage        string `gorm:"type:varchar(1024)"`
	HashedPassword string `gorm:"type:varchar(1024);not null"`
	IsActive       bool   `gorm:"default:true;not null"`
	IsSuperuser    bool   `gorm:"default:false;not null"`
	IsVerified     bool   `gorm:"default:false;not null"`
	Tracks         []Track
	Playlists      []Playlist
}
