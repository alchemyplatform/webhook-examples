# Example Alchemy Notify Webhooks Server

A simple example webhook server for using Alchemy Notify that uses python django.

## Installation

#### Simple setup

With this command, create & source your virtual environment and install your dependencies:

```
python3 -m venv env && source env/bin/activate && pip install -r requirements.txt
```

## Run

First, access the server directory:

```
cd webhook_server
```

To run on localhost:8080:

```
cd webhook_server

SIGNING_KEY=whsec_your_key_here python manage.py runserver localhost:8080
```

And just like that, you're done!

NOTE: Your webhook path is currently set to "webhook-path" in `webhook_server/backend/urls.py`, but feel free to change it to whatever path you'd like.

## Debugging

If you aren't receiving any webhooks, be sure you followed the steps [here first](https://github.com/alchemyplatform/#readme).
