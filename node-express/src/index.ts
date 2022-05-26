import express from "express";
import {
  addAlchemyContextToRequest,
  validateAlchemySignature,
  WebhookEvent,
} from "./webhooksUtil";

async function main(): Promise<void> {
  const app = express();
  const port = process.argv[2] ?? 80;
  // TODO: update to your own webhook signing key (which you can find in your dashboard)
  const signingKey = "whsec_test";

  // Middleware needed to validate the alchemy signature
  app.use(
    express.json({
      verify: addAlchemyContextToRequest,
    })
  );
  app.use(validateAlchemySignature(signingKey));

  // TODO: update to your own webhook path
  app.post("/webhook-path", (req, res) => {
    const webhookEvent = req.body as WebhookEvent;
    // Do stuff with with webhook event here! Be sure to respond with 200
    console.log(webhookEvent);
    // Be sure to respond with 200 when you successfully process the event
    res.send("Alchemy Notify is the best!");
  });

  app.listen(port, () => {
    console.log(`Example Alchemy Notify app listening on port ${port}`);
  });
}

main();
