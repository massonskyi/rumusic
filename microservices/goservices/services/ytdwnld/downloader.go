package ytdwnld

import (
	"fmt"
	"os/exec"
)

func DownloadFromYoutube(url string) (string, error) {
	// Using yt-dlp for example
	cmd := exec.Command("yt-dlp", url, "-o", "/tmp/%(title)s.%(ext)s")
	output, err := cmd.CombinedOutput()
	if err != nil {
		return "", fmt.Errorf("error downloading video: %v, output: %s", err, string(output))
	}

	// Assuming the output file is located in /tmp directory
	// Extracting file path from output could be necessary depending on yt-dlp configuration
	return "/tmp/some_video.mp4", nil
}
