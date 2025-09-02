package main

import "net/http"
import "log"
import "fmt"

// Port that the application runs on
const Port = 9080

func main() {
	fmt.Println("textsender-auth")

	portListen := fmt.Sprintf(":%d", Port)

	log.Println("Starting server port:", Port)
	log.Fatal(http.ListenAndServe(portListen, nil))
}
