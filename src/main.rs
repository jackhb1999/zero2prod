use std::net::TcpListener;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use zero2prod::{
    configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    // let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string())
    //     .expect("Failed to connect to database");
    let connection_pool = PgPoolOptions::new()
        .connect_lazy_with(configuration.database.with_db());
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("Server is running on  http://{}:{}", configuration.application.host, port);
    run(listener, connection_pool)?.await?;
    Ok(())
}
