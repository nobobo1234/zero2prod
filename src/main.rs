use std::net::TcpListener;
use sqlx::PgPool;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

// The tokio::main macro allows us to have an async main function by adding
// scaffolding code in the runtime
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Read the configuration into a variable and panic if we can't read it.
    let configuration = get_configuration().expect("Failed to read config.");
    let connection_pool = PgPool::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port); 
    let listener = TcpListener::bind(address)
        .expect("Failed to bind to port");

    // Show the io::Error if we cannot bind to the address, otherwise call
    // .await on our Server
    run(listener, connection_pool)?.await
}
