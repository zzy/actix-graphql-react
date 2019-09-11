use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use futures::future::Future;

use juniper::http::playground::playground_source;
use juniper::{http::GraphQLRequest, Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

use diesel::prelude::*;

use crate::db::{DbCon, DbPool};

graphql_schema_from_file!("src/schema.graphql");

pub struct Context {
    db_con: DbCon,
}
impl juniper::Context for Context {}

pub struct Query;
pub struct Mutation;

impl QueryFields for Query {
    fn field_cats(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Cat, Walked>,
        skip: i32,
        limit: i32,
    ) -> FieldResult<Vec<Cat>> {
        use crate::schema::cats;

        let result = cats::table
            .offset(skip.into())
            .limit(limit.into())
            .load::<crate::models::Cat>(&_executor.context().db_con)
            .expect("Error loading posts");

        Ok(result
            .iter()
            .map(|cat| Cat {
                id: cat.id,
                name: cat.name.to_owned(),
            })
            .collect())
    }
}

impl MutationFields for Mutation {
    fn field_add_cat(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Cat, Walked>,
        name: String,
    ) -> FieldResult<Cat> {
        use crate::schema::cats;

        let new_cat = crate::models::NewCat { name: name };

        let cat: crate::models::Cat = diesel::insert_into(cats::table)
            .values(&new_cat)
            .get_result(&_executor.context().db_con)
            .expect("Error saving new post");

        Ok(Cat {
            id: cat.id,
            name: cat.name.to_owned(),
        })
    }
}

pub struct Cat {
    id: i32,
    name: String,
}

impl CatFields for Cat {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<juniper::ID> {
        Ok(juniper::ID::new(self.id.to_string()))
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }
}

fn graphiql() -> HttpResponse {
    let html = playground_source("");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    db_pool: web::Data<DbPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let ctx = Context {
        db_con: db_pool.get().unwrap(),
    };

    web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

pub fn register(config: &mut web::ServiceConfig) {
    let schema = std::sync::Arc::new(Schema::new(Query, Mutation));

    config
        .data(schema)
        .route("/gql", web::post().to_async(graphql))
        .route("/giql", web::get().to(graphiql));
}
