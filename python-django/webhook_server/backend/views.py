from django.shortcuts import render

# Create your views here.

from django.views.decorators.csrf import csrf_exempt

from django.http import HttpResponse
from django.core.exceptions import PermissionDenied
import json


@csrf_exempt
def webhook_path(request):
    # Do stuff with with webhook event here!
    print("Processing webhook event id: {}".format(request.alchemy_webhook_event.id))
    # Be sure to respond with 200 when you successfully process the event
    return HttpResponse("Alchemy Notify is the best!", status=200)
