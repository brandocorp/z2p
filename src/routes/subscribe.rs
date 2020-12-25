use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeData {
  email: String,
  name: String,
}

#[tracing::instrument(
  name = "subscribe",
  skip(form, pool),
  fields(
    email = %form.email,
    name = %form.name
  )
)]
pub async fn subscribe(
  form: web::Form<SubscribeData>,
  pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
  insert_subscriber(&pool, &form)
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;
  Ok(HttpResponse::Ok().finish())
}


#[tracing::instrument(
  name = "insert_subscriber",
  skip(form, pool)
)]
pub async fn insert_subscriber(
  pool: &PgPool,
  form: &SubscribeData,
) -> Result<(), sqlx::Error> {
  sqlx::query!(
    r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now()
  )
  .execute(pool)
  .await
  .map_err(|e| {
    tracing::error!("Failed to execute query: {:?}", e);
    e
  })?;

  Ok(())
}