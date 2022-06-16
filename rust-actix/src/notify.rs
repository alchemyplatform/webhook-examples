use chrono::{DateTime, FixedOffset};
use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use serde::{de, Deserialize, Deserializer};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorBadRequest,
    error::ErrorUnauthorized,
    web::BytesMut,
    Error, HttpMessage,
};
use futures_util::{future::LocalBoxFuture, stream::StreamExt};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct AlchemyWebhookEvent {
    pub webhook_id: String,
    pub id: String,
    #[serde(deserialize_with = "deserialize_date_time")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "type")]
    pub webhook_type: String,
    pub event: serde_json::Value,
}

fn deserialize_date_time<'a, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'a>,
{
    let date_time_string: String = Deserialize::deserialize(deserializer)?;
    let date_time = DateTime::<FixedOffset>::parse_from_rfc3339(&date_time_string)
        .map_err(|e| de::Error::custom(e.to_string()))?;
    Ok(date_time)
}

fn is_valid_signature_for_string_body(
    body: &[u8],
    signature: &str,
    signing_key: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let signing_key_bytes: Vec<u8> = signing_key.bytes().collect();
    let mut mac = Hmac::<Sha256>::new_from_slice(&signing_key_bytes)?;
    mac.update(&body);
    let hex_decode_signature = hex::decode(signature)?;
    let verification = mac.verify_slice(&hex_decode_signature).is_ok();
    Ok(verification)
}

pub struct AlchemyRequestHandlerMiddleware<S> {
    signing_key: String,
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AlchemyRequestHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let signing_key = self.signing_key.clone();

        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            let signature = req
                .headers()
                .get("x-alchemy-signature")
                .ok_or(ErrorBadRequest(
                    "Signature validation failed, missing x-alchemy-signature header!",
                ))?
                .to_str()
                .map_err(|_| {
                    ErrorBadRequest(
                        "Signature validation failed, x-alchemy-signature header is not a string!",
                    )
                })?;

            let is_valid_signature =
                is_valid_signature_for_string_body(&body, signature, &signing_key)?;

            if !is_valid_signature {
                return Err(ErrorUnauthorized(
                    "Signature validation failed, signature and body do not match!",
                ));
            }

            let webhook: AlchemyWebhookEvent = serde_json::from_slice(&body).map_err(|_| {
                ErrorBadRequest("Bad format, body could not be mapped to AlchemyWebhookEvent")
            })?;

            req.extensions_mut()
                .insert::<Rc<AlchemyWebhookEvent>>(Rc::new(webhook));

            let res = svc.call(req).await?;

            Ok(res)
        })
    }
}

pub struct AlchemyRequestHandlerMiddlewareFactory {
    signing_key: String,
}

impl AlchemyRequestHandlerMiddlewareFactory {
    pub fn new(signing_key: String) -> Self {
        AlchemyRequestHandlerMiddlewareFactory { signing_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AlchemyRequestHandlerMiddlewareFactory
where
    B: 'static,
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AlchemyRequestHandlerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AlchemyRequestHandlerMiddleware {
            signing_key: self.signing_key.clone(),
            service: Rc::new(service),
        }))
    }
}
