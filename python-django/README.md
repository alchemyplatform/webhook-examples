# Example Alchemy Notify Webhooks Server

A simple example webhook server for using Alchemy Notify that uses python django.

## Installation

#### Simple setup

With this command, create & source your virtual environment and install your dependencies:

```
python3 -m venv env && source env/bin/activate && pip install -r requirements.txt
```

## Run

To run on localhost:8080:

```
cd webhook_server

python manage.py runserver localhost:8080
```

And just like that, you're done!

Note: there are at least two hard coded variables in that you will have to modify to fit your needs:

- `signing_key`: your webhook signing key, currently set to "whsec_test" in `webhook_server/backend/urls.py`, please change it your actual signing key which you can find [here](https://docs.alchemy.com/alchemy/enhanced-apis/notify-api/using-notify#1.-find-your-signing-key)

- `path`: your webhook path, currently set to "/webhook-path" in `webhook_server/backend/urls.py`, but feel free to change it to whatever path you'd like

## Test Locally

To test your webhook locally, we recommend using ngrok and following the guide [here](https://docs.alchemy.com/alchemy/enhanced-apis/notify-api/using-notify#1.-find-your-signing-key). NOTE: When you fire up your local forwarding tunnel be sure to use the same port as the server above.

Example on port 8080:

`ngrok http 8080`
