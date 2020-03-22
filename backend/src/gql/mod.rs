
use std::sync::Arc;
use juniper::RootNode;
use juniper::http::GraphQLRequest;
use juniper::http::playground::playground_source;
use actix_web::{ web, Error, HttpResponse };

use crate::data::db::PostgresPool;

pub mod context;
pub mod queries;
pub mod mutations;

use context::GraphQLContext;
use queries::QueryRoot;
use mutations::MutationRoot;

// The root schema that pulls the query and mutation together.
// Perhaps someday you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}

// The configuration callback that enables us to add the /gql route
// to the actix-web server.
pub fn endpoints(config: &mut web::ServiceConfig) {
    let schema = Arc::new(create_schema());
    config
        .data(schema)
        .service(
            web::resource("/gql")
                .route(web::post().to(graphql))
                .route(web::get().to(playground))
                // .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
        );
        // .route("/gql", web::post().to(graphql))
        // .route("/gql", web::get().to(playground));
}

// The GraphQL Playground route.
async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/gql"))
}

// The core handler that provides all GraphQL functionality.
async fn graphql(
    // The DB connection pool
    pool: web::Data<PostgresPool>,
    // The GraphQL schema
    schema: web::Data<Arc<Schema>>,
    // The incoming HTTP request
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    // Instantiate a context
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
    };

    // Handle the incoming request and return a string result (or error)
    let res = web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(Error::from)?;

    // Return the string as a JSON payload
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}
