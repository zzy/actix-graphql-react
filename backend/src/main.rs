// #![deny(warnings)]

use std::io;
use std::sync::Arc;

extern crate diesel;
extern crate juniper;
extern crate pretty_env_logger;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

extern crate actix_web_juniper_react_apollo;
use actix_web_juniper_react_apollo::gql::{Context, Mutation, Query};
// use actix_web_juniper_react_apollo::jwt::verify_jwt;

// use warp::{fiactix_web_juniper_react_apollolters::BoxedFilter, Filter};

type Schema = juniper::RootNode<'static, Query, Mutation>;

fn main() -> io::Result<()> {
    pretty_env_logger::init();
    std::env::set_var("RUST_LOG", "actix_web=info");

    // Create Juniper schema
    // let schema = std::sync::Arc::new(create_schema());
    let schema = Schema::new(Query, Mutation);

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:5000")?
    .run()
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:5000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
