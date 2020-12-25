use sqlx::postgres::PgPool;
use z2p::configuration::get_configuration;
use z2p::startup::run;
use z2p::telemetry::{get_subscriber, init_subscriber};

use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    init_subscriber(get_subscriber("z2p".into(), "info".into()));
    

    let config = get_configuration()
        .expect("Failed to read configuration.");

    let pool = PgPool::connect(&config.db.connection_string())
        .await
        .expect("Failed to connect to postgres.");
    
    let address = format!("{}:{}", config.api.host, config.api.port);
    let listener = TcpListener::bind(address)
        .expect("Failed to bind port 8080");

    run(listener, pool)?.await
}