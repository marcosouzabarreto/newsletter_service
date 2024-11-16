mod routes;
mod startup;
use newsletter_service::{
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::PgPool;
use startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletter_service".to_string(), "info".to_string());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to read configuration");

    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Error to bind random port");
    run(listener, connection_pool)?.await
}
