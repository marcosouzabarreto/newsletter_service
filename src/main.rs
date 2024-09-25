mod routes;
mod startup;
use newsletter_service::configuration::get_configuration;
use startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to get configuration");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Error to bind random port");
    run(listener)?.await
}
