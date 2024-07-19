package ytdwnld

import (
	"encoding/json"
	"net/http"
)

// DownloadRequest represents the payload for download request
type DownloadRequest struct {
	URL string `json:"url"`
}

// DownloadResponse represents the response with file path
type DownloadResponse struct {
	FilePath string `json:"file_path"`
}

// DownloadVideo godoc
// @Summary Download a video from YouTube
// @Description Download a video from YouTube using the provided URL
// @Tags download
// @Accept  json
// @Produce  json
// @Param   request body DownloadRequest true "Download Request"
// @Success 200 {object} DownloadResponse
// @Failure 400 {string} string "Invalid request payload"
// @Failure 500 {string} string "Internal Server Error"
// @Router /download [post]
func DownloadVideo(w http.ResponseWriter, r *http.Request) {
	var req DownloadRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request payload", http.StatusBadRequest)
		return
	}

	// Simulated response
	resp := DownloadResponse{FilePath: "/tmp/some_video.mp4"}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}
