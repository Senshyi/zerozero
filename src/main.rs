use sqlx::PgPool;
use std::net::TcpListener;

use zerozero::configuration::get_configuration;
use zerozero::startup::run;
use zerozero::telemetry::{init_subscriber,get_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zerozero".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connect_string())
        .await
        .expect("Failed to connect to postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await?;
    Ok(())
}
