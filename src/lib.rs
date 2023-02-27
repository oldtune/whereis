mod entities;
mod infras;
mod route_data;
mod routes;
mod services;
use actix_web::{App, HttpServer};

pub async fn run_app(port: usize) -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::location::calculate_distance))
        .bind(("localhost", 8080))?
        .run()
        .await;
    Ok(())
}
