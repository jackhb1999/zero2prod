use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use sqlx::{PgConnection, PgPool};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let request_id = Uuid::new_v4();
    // let request_span = tracing::info_span!(
    // "Adding a new subscriber.",%request_id,subscriber_email = %from.email,
    // subscriber_name = %from.name
    // );
    // let _request_span_guard = request_span.enter();

    // if !is_valid_name(&form.name) {
    //     return HttpResponse::BadRequest().finish();
    // }

    // let name = match SubscriberName::parse(form.0.name) {
    //     Ok(name) => name,
    //     Err(_) => return HttpResponse::BadRequest().finish(),
    // };
    //
    // let email = match SubscriberEmail::parse(form.0.email) {
    //     Ok(email) => email,
    //     Err(_) => return HttpResponse::BadRequest().finish(),
    // };
    //
    // let new_subscriber = NewSubscriber {
    //     email,
    //     // name:SubscriberName::parse(form.0.name).expect("invalid subscriber name"),
    //     name,
    // };

    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(err) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => {
            // log::info!("New subscription: {}", from.email);
            // tracing::info!("New subscription: {}", from.email);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // log::error!("Failed to execute query: {}", e);
            tracing::error!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

pub fn is_valid_name(name: &str) -> bool {
    let is_empty_or_whitespace = name.trim().is_empty();
    let is_too_long = name.graphemes(true).count() > 256;

    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = name.chars().any(|g| forbidden_characters.contains(&g));
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &NewSubscriber) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id,email, name,subscribed_at)
    VALUES ($1, $2,$3,$4)
    "#,
        Uuid::new_v4(),
        form.email.as_ref(),
        form.name.as_ref(),
        chrono::Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
