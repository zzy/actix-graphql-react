use std::{ env, io };
use dotenv::dotenv;
use actix_web::{ App, HttpServer, http, middleware };
use actix_cors::Cors;

// agract_backend is the name defined in the cargo.toml.
use agract_backend::data::db::pg_pool;
use agract_backend::gql::endpoints;

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

    // let cors_uri: String = env::var("CORS_URI")
    //     .unwrap_or_else(|_| String::from("http://127.0.0.1:3000"));
    
    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b)
    // the configuration function that adds the /gql logic.
    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    // .allowed_origin(&cors_uri)  // <- Defaults to `All`
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    // .supports_credentials()
                    .max_age(3600)
                    .finish()
            )
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
