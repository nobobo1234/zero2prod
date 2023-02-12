use std::net::TcpListener;
use actix_web::{HttpServer, App, web}; 
use actix_web::dev::Server;
use sqlx::PgPool;
use crate::routes::{health_check, subscribe};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    // Wrap our connection in an Arc<T> so that it is clonable
    let db_pool = web::Data::new(db_pool);
        
    // Generate a new HttpServer and add an App to it with routes
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    
    Ok(server)
}
