package models

type Playlist struct {
	ID     uint   `gorm:"primary_key"`
	Name   string `gorm:"not null"`
	UserID uint
	Tracks []Track `gorm:"many2many:playlist_tracks"`
}
