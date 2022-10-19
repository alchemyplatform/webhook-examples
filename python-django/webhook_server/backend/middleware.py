import hmac
import hashlib
from django.core.exceptions import PermissionDenied
from webhook_server.settings import SIGNING_KEY
import json
from types import SimpleNamespace


def is_valid_signature_for_string_body(
    body: str, signature: str, signing_key: str
) -> bool:
    digest = hmac.new(
        bytes(signing_key, "utf-8"),
        msg=bytes(body, "utf-8"),
        digestmod=hashlib.sha256,
    ).hexdigest()

    return signature == digest


class AlchemyWebhookEvent:
    def __init__(self, webhookId, id, createdAt, type, event):
        self.webhook_id = webhookId
        self.id = id
        self.created_at = createdAt
        self.type = type
        self.event = event


class AlchemyRequestHandlerMiddleware:
    def __init__(self, get_response):
        self.get_response = get_response

    def __call__(self, request):
        str_body = str(request.body, request.encoding or "utf-8")
        signature = request.headers["x-alchemy-signature"]
        if not is_valid_signature_for_string_body(str_body, signature, SIGNING_KEY):
            raise PermissionDenied("Signature validation failed, unauthorized!")

        webhook_event = json.loads(str_body)
        request.alchemy_webhook_event = AlchemyWebhookEvent(**webhook_event)
        response = self.get_response(request)
        return response
