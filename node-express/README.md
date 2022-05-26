# Example Alchemy Notify Webhooks Server

A simple example webhook server for using Alchemy Notify that uses node express.

## Installation

#### Simple setup

First, install Yarn if you don't have it:

```
npm install -g yarn
```

Then, install the dependencies of all packages:

```
yarn
```

## Run

To run on port 80:

```
yarn start 80
```

And just like that, you're done!

Note: there are at least three hard coded variables in that you will have to modify to fit your needs:

- `signingKey`: your webhook signing key, currently set to "whsec_test" in `index.ts`, please change it your actual signing key which you can find [here](https://docs.alchemy.com/alchemy/enhanced-apis/notify-api/using-notify#1.-find-your-signing-key)

- `path`: your webhook path, currently set to "/webhook-path" in `index.ts`, but feel free to change it to whatever path you'd like

## Test Locally

To test your webhook locally, we recommend using ngrok and following the guide [here](https://docs.alchemy.com/alchemy/enhanced-apis/notify-api/using-notify#1.-find-your-signing-key). NOTE: When you fire up your local forwarding tunnel be sure to use the same port as the server above.

Example on port 80:

`ngrok http 80`
