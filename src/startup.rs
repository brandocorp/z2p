use super::routes::{health, subscribe};

use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;

use std::net::TcpListener;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
  let db = web::Data::new(pool);
  let server = HttpServer::new(move || {
      App::new()
        .route("/health", web::get().to(health))
        .route("/subscribe", web::post().to(subscribe))
        .app_data(db.clone())
    })
    .listen(listener)?
    .run();

  Ok(server)
}