package main

import (
	"fmt"
	"log"
	"net/http"
	"os"

	notify "github.com/alchemyplatform/alchemy-notify-sdk-go"
)

func setDefaultEnvVar(k string, v string) {
	if _, ok := os.LookupEnv(k); !ok {
		os.Setenv(k, v)
	}
}

func handleWebhook(w http.ResponseWriter, req *http.Request, event *notify.AlchemyWebhookEvent) {
	// Do stuff with with webhook event here!
	log.Printf("Processing webhook event id: %s\n", event.Id)
	// Be sure to respond with 200 when you successfully process the event
	w.Write([]byte("Alchemy Notify is the best!"))
}

func main() {
	setDefaultEnvVar("PORT", "8080")
	setDefaultEnvVar("HOST", "127.0.0.1")
	setDefaultEnvVar("SIGNING_KEY", "whsec_test")

	port := os.Getenv("PORT")
	host := os.Getenv("HOST")
	signingKey := os.Getenv("SIGNING_KEY")

	mux := http.NewServeMux()

	// Register handler for Alchemy Notify webhook events
	mux.Handle(
		// TODO: update to your own webhook path
		"/webhook-path",
		// Middleware needed to validate the alchemy signature
		notify.NewAlchemyRequestHandlerMiddleware(handleWebhook, signingKey),
	)

	// Listen to Alchemy Notify webhook events
	addr := fmt.Sprintf("%s:%s", host, port)
	log.Printf("Example Alchemy Notify app listening at %s\n", addr)
	err := http.ListenAndServe(addr, mux)
	log.Fatal(err)
}
