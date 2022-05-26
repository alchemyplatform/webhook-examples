from django.shortcuts import render

# Create your views here.

from django.views.decorators.csrf import csrf_exempt

from .utils.webhook_utils import is_valid_signature_for_string_body
from django.http import HttpResponse
from django.core.exceptions import PermissionDenied
import json

# TODO: update to your own webhook signing key (which you can find in your dashboard)
signing_key = "whsec_test"

# Using Django
@csrf_exempt
def webhook_path(request):
    str_body = str(request.body, request.encoding or "utf-8")
    signature = request.headers["x-alchemy-signature"]

    if not is_valid_signature_for_string_body(str_body, signature, signing_key):
        raise PermissionDenied("Signature validation failed, unauthorized!")
    else:
        webhook_event = json.loads(str_body)
        # Do stuff with with webhook event here! Be sure to respond with 200
        print(webhook_event)
        # Be sure to respond with 200 when you successfully process the event
        return HttpResponse("Alchemy Notify is the best!", status=200)
