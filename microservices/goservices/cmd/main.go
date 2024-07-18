package main

import (
	"backend_engine/config"
	"backend_engine/db"
	"backend_engine/routes"
	"github.com/jinzhu/gorm"
	"net/http"

	"github.com/gorilla/mux"
)

func main() {
	config.LoadConfig()
	db.Connect()
	defer func(DB *gorm.DB) {
		err := DB.Close()
		if err != nil {

		}
	}(db.DB)

	router := mux.NewRouter()
	routes.SetupRoutes(router)

	err := http.ListenAndServe(":8080", router)
	if err != nil {
		return
	}
	println("Server started on port 8080")
}
