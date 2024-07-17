package models

import (
	"os"
	"time"
)

func (u *User) CreateStoragePath() string {
	storagePath := "storage/user_storage_" + u.Email + "_" + time.Now().Format("2006-01-02_15-04-05")
	err := os.MkdirAll(storagePath, os.ModePerm)
	if err != nil {
		panic(err)
	}
	return storagePath
}

// GetStoragePath returns the storage path of the user.
func (u *User) GetStoragePath() string {
	return u.Storage
}

// GetUserID returns the user's id.
func (u *User) GetUserID() uint {
	return u.ID
}

// GetUserName returns the user's name.
func (u *User) GetUserName() string {
	return u.Name
}

// GetUserEmail returns the user's email.
func (u *User) GetUserEmail() string {
	return u.Email
}

// GetUserUsername returns the user's username.
func (u *User) GetUserUsername() string {
	return u.Username
}

// GetUserPassword returns the user's hashed password.
func (u *User) GetUserPassword() string {
	return u.HashedPassword
}

// GetUserIsActive returns the user's is_active status.
func (u *User) GetUserIsActive() bool {
	return u.IsActive
}

// GetUserIsSuperuser returns the user's is_superuser status.
func (u *User) GetUserIsSuperuser() bool {
	return u.IsSuperuser
}

// GetUserIsVerified returns the user's is_verified status.
func (u *User) GetUserIsVerified() bool {
	return u.IsVerified
}
