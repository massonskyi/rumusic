package controllers

import (
	"backend_engine/services"
	"backend_engine/utils"
	"encoding/json"
	"net/http"

	"github.com/jinzhu/gorm"
)

func Register(db *gorm.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var user struct {
			Username string `json:"username"`
			Password string `json:"password"`
		}

		err := json.NewDecoder(r.Body).Decode(&user)
		if err != nil {
			return
		}

		err = services.RegisterUser(db, user.Username, user.Password)

		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		w.WriteHeader(http.StatusCreated)
	}
}

func Login(db *gorm.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var user struct {
			Username string `json:"username"`
			Password string `json:"password"`
		}

		err := json.NewDecoder(r.Body).Decode(&user)

		if err != nil {
			return
		}

		auth, err := services.AuthenticateUser(db, user.Username, user.Password)
		if err != nil || !auth {
			http.Error(w, "Invalid credentials", http.StatusUnauthorized)
			return
		}

		token, err := utils.GenerateJWT(user.Username)

		if err != nil {
			http.Error(w, "Error generating token", http.StatusInternalServerError)
			return
		}

		err = json.NewEncoder(w).Encode(map[string]string{"token": token})
		if err != nil {
			return
		}
	}
}
