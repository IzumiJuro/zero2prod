use std::{io::Result, net::TcpListener};
use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};
#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect Postgres.");
    let port = configuration.application_port;
    let address = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(address)?;
    run(listener, connection.clone())?.await
}
