package routes

import (
	"backend_engine/controllers"
	"backend_engine/db"
	"backend_engine/utils"
	"context"
	"net/http"

	"github.com/gorilla/mux"
)

func SetupRoutes(router *mux.Router) {
	db_ := db.DB

	router.HandleFunc("/register", controllers.Register(db_)).Methods("POST")
	router.HandleFunc("/login", controllers.Login(db_)).Methods("POST")

	//authRouter := router.PathPrefix("/songs").Subrouter()
	//authRouter.Use(utils.JWTMiddleware)
	//authRouter.HandleFunc("", controllers.CreateSong(db_)).Methods("POST")
	//authRouter.HandleFunc("", controllers.GetSongs(db_)).Methods("GET")
}

func JWTMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		tokenStr := r.Header.Get("Authorization")
		claims, valid := utils.ValidateJWT(tokenStr)
		if !valid {
			http.Error(w, "Invalid token", http.StatusUnauthorized)
			return
		}
		ctx := context.WithValue(r.Context(), "userID", claims.Username)
		next.ServeHTTP(w, r.WithContext(ctx))
	})
}
