use z2p::configuration::{DatabaseSettings, get_configuration};
use z2p::startup::run;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use std::net::TcpListener;

pub struct TestApp {
  pub address: String,
  pub db: PgPool,
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
  let mut connection = PgConnection::connect(&config.connection_string_without_db())
    .await
    .expect("Failed to connect to postgres.");
  connection
    .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database))
    .await
    .expect("Failed to create database.");
  
  let connection_pool = PgPool::connect(&config.connection_string())
    .await
    .expect("Failed to connect to postgres.");
  
  sqlx::migrate!("./migrations")
    .run(&connection_pool)
    .await
    .expect("Failed to migrate the database.");
  
  connection_pool
}

async fn spawn_app() -> TestApp {
  let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port.");
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", port);

  let mut config = get_configuration()
    .expect("Failed to read configuration.");
  config.db.database = Uuid::new_v4().to_string();

  let pool = configure_database(&config.db).await;

  let server = run(listener, pool.clone())
    .expect("Failed to bind address.");
  let _ = tokio::spawn(server);

  TestApp {
    address: address,
    db: pool,
  }
}

#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
  // Arrange
  let app = spawn_app().await;
  let endpoint = format!("{}/subscribe", &app.address);
  let client = reqwest::Client::new();
  let body = "name=Bob%20Spam&email=f%40ke.com";

  // Act
  let response = client
    .post(&endpoint)
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert
  assert_eq!(200, response.status().as_u16());

  let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&app.db)
    .await
    .expect("Failed to fetch saved subscription.");
  
  assert_eq!(saved.email, "f@ke.com");
  assert_eq!(saved.name, "Bob Spam");
}

#[actix_rt::test]
async fn subscribe_returns_400_with_invalid_data() {
  // Arrange
  let app = spawn_app().await;
  let endpoint = format!("{}/subscribe", &app.address);
  let client = reqwest::Client::new();

  let test_cases = vec![
    ("name=Bob%20Spam", "missing email"),
    ("email=f%40ke.com", "missing name"),
    ("", "missing name and email")
  ];


  for (invalid_body, error_message) in test_cases {
    // Act
    let response = client
      .post(&endpoint)
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(invalid_body)
      .send()
      .await
      .expect("Failed to execute request.");

    // Assert
    assert_eq!(
      400, 
      response.status().as_u16(),
      "The server did not respond with 400 bad when body was {}.",
      error_message
    );
  }
}

#[actix_rt::test]
async fn get_health_succeeds() {
  // Arange
  let app = spawn_app().await;
  let endpoint = format!("{}/health", &app.address);
  let client = reqwest::Client::new();

  // Act
  let response = client
    .get(&endpoint)
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

