# Example Alchemy Notify Webhooks Server in Rust

A simple example webhook server for using Alchemy Notify that uses rust [actix](https://actix.rs/docs/getting-started/).

## Run

To run on localhost:8080:

```
PORT=8080 HOST=localhost SIGNING_KEY=whsec_your_key_here cargo run
```

Please change SIGNING_KEY to the signing key corresponding to your webhook, which you can find [here](https://docs.alchemy.com/alchemy/enhanced-apis/notify-api/using-notify#1.-find-your-signing-key)

And just like that, you're done!

NOTE: Your webhook path is currently set to "/webhook-path" in `src/main.rs`, but feel free to change it to whatever path you'd like.

## Debugging

If you aren't receiving any webhooks, be sure you followed the steps [here first](https://github.com/alchemyplatform/#readme).
