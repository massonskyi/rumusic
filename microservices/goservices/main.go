package main

//go:generate swag init -g main.go

import (
	"backend_engine/services/ytdwnld"
	"github.com/gorilla/mux"
	httpSwagger "github.com/swaggo/http-swagger"
	"log"
	"net/http"
)

// @title YouTube Downloader API
// @version 1.0
// @description This is a sample server for downloading YouTube videos.
// @host localhost:8080
// @BasePath /

func main() {
	router := mux.NewRouter()
	router.HandleFunc("/download", ytdwnld.DownloadVideo).Methods("POST")

	// Swagger endpoint
	router.PathPrefix("/swagger/").Handler(httpSwagger.WrapHandler)

	log.Println("Starting server on :8080")
	if err := http.ListenAndServe(":8080", router); err != nil {
		log.Fatal("ListenAndServe:", err)
	}
}
