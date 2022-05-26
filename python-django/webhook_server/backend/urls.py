from django.urls import path

from . import views

# TODO: update to your own webhook path
urlpatterns = [
    path("webhook-path", views.webhook_path, name="webhook_path"),
]
