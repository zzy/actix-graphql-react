use std::{env, io};
use dotenv::dotenv;
use actix_web::{middleware, App, HttpServer};

// agract is the name defined in the cargo.toml.
use agract::data::db::pg_pool;
use agract::graphql::endpoints;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    logging_setup();

    // Instantiate a new connection pool
    let pool = pg_pool();

    let graphql_port: i16 = env::var("GRAPHQL_PORT")
        .unwrap_or_else(|_| String::from("5000"))
        .parse()
        .expect("GRAPHQL_PORT must be a number");

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(endpoints)
    })
    .bind(format!("0.0.0.0:{}", graphql_port))?
    .run();

    eprintln!("Listening on 0.0.0.0:{}", graphql_port);

    server.await
}

// TODO: more fine-grained logging setup
fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
