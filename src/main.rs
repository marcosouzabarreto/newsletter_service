mod routes;
mod startup;
use env_logger::Env;
use newsletter_service::configuration::get_configuration;
use sqlx::PgPool;
use startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to read configuration");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Error to bind random port");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    run(listener, connection_pool)?.await
}
