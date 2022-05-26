import hmac
import hashlib


def is_valid_signature_for_string_body(body: str, signature: str, signing_key: str):
    digest = hmac.new(
        bytes(signing_key, "utf-8"), msg=bytes(body, "utf-8"), digestmod=hashlib.sha256
    ).hexdigest()

    return signature == digest
