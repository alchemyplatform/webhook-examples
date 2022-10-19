package notify

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
	"time"
)

type AlchemyWebhookEvent struct {
	WebhookId string
	Id        string
	CreatedAt time.Time
	Type      string
	Event     map[string]interface{}
}

func jsonToAlchemyWebhookEvent(body []byte) (*AlchemyWebhookEvent, error) {
	event := new(AlchemyWebhookEvent)
	if err := json.Unmarshal(body, &event); err != nil {
		return nil, err
	}
	return event, nil
}

// Middleware helpers for handling an Alchemy Notify webhook request
type AlchemyRequestHandler func(http.ResponseWriter, *http.Request, *AlchemyWebhookEvent)

type AlchemyRequestHandlerMiddleware struct {
	handler    AlchemyRequestHandler
	signingKey string
}

func NewAlchemyRequestHandlerMiddleware(handler AlchemyRequestHandler, signingKey string) *AlchemyRequestHandlerMiddleware {
	return &AlchemyRequestHandlerMiddleware{handler, signingKey}
}

func isValidSignatureForStringBody(
	body []byte,
	signature string,
	signingKey []byte,
) bool {
	h := hmac.New(sha256.New, signingKey)
	h.Write([]byte(body))
	digest := hex.EncodeToString(h.Sum(nil))
	return digest == signature
}

func (arh *AlchemyRequestHandlerMiddleware) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	signature := r.Header.Get("x-alchemy-signature")
	body, err := ioutil.ReadAll(r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		log.Panic(err)
		return
	}
	r.Body.Close()

	isValidSignature := isValidSignatureForStringBody(body, signature, []byte(arh.signingKey))
	if !isValidSignature {
		errMsg := "Signature validation failed, unauthorized!"
		http.Error(w, errMsg, http.StatusForbidden)
		log.Panic(errMsg)
		return
	}

	event, err := jsonToAlchemyWebhookEvent(body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		log.Panic(err)
		return
	}
	arh.handler(w, r, event)
}
