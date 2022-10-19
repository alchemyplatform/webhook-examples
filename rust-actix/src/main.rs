mod env_helpers;
mod notify;

use log::info;

use actix_web::{middleware, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer};
use std::{io, rc::Rc};

use crate::env_helpers::cast_required_env_var;
use crate::env_helpers::set_default_env_var;
use crate::notify::AlchemyWebhookEvent;

async fn webhook_handler(req: HttpRequest) -> HttpResponse {
    let extensions = req.extensions();
    let event = extensions
        .get::<Rc<AlchemyWebhookEvent>>()
        .unwrap()
        .as_ref();
    // Do stuff with with webhook event here!
    info!("Processing webhook event id: {:?}", event.id);
    // Be sure to respond with 200 when you successfully process the event
    HttpResponse::Ok().body("Alchemy Notify is the best!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    set_default_env_var("RUST_LOG", "info");
    env_logger::init();

    set_default_env_var("PORT", "8080");
    set_default_env_var("HOST", "127.0.0.1");
    set_default_env_var("SIGNING_KEY", "whsec_test");

    let port = cast_required_env_var::<u16>("PORT");
    let host = cast_required_env_var::<String>("HOST");
    let signing_key = cast_required_env_var::<String>("SIGNING_KEY");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // Middleware needed to validate the alchemy signature
            .wrap(notify::AlchemyRequestHandlerMiddlewareFactory::new(
                signing_key.clone(),
            ))
            // Register handler for Alchemy Notify webhook events
            .service(
                web::resource(
                    // TODO: update to your own webhook path
                    "/webhook-path",
                )
                .to(webhook_handler),
            )
    })
    // Listen to Alchemy Notify webhook events
    .bind((host, port))?
    .run()
    .await
}
