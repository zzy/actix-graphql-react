use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use actix_graphql_react_apollo as agra;

fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let db_pool = agra::db::create_db_pool();
    let gql_server_port: u16 = std::env::var("GRAPHQL_SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], gql_server_port));

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(Cors::new())
            .configure(agra::gql::register)
            .default_service(web::to(|| "404"))
    })
    .bind(addr)
    .unwrap()
    .run()
    .unwrap();
}
