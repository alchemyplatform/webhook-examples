from django.urls import path

from . import views

# Register handler for Alchemy Notify webhook events
# TODO: update to your own webhook path
urlpatterns = [
    path("webhook-path", views.webhook_path, name="webhook_path"),
]
