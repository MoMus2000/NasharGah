package main

import (
	"net/http"
	"path/filepath"
        "fmt"
)

func serveHomepage(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")

	// Replace the file path with the actual path to your HTML file
	htmlFilePath := "/Users/mmuhammad/Desktop/projects/nashar_gah/assets/index.html"

	// Read the file and serve its content
	http.ServeFile(w, r, filepath.Clean(htmlFilePath))
}

func main() {
	http.HandleFunc("/", serveHomepage)
	fmt.Println("Serving @localhost:8080")
	http.ListenAndServe(":8080", nil)
}
