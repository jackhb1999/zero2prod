use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use sqlx::{PgConnection, PgPool};
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

    match insert_subscriber(&pool, &form).await {
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

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id,email, name,subscribed_at)
    VALUES ($1, $2,$3,$4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
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
