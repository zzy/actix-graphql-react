use juniper::{FieldResult, RootNode};
use actix::prelude::*;
use actix_web::{AsyncResponder, HttpResponse};
use db::{DbExecutor, CreateUser, GetUser};
use futures::future::Future;

#[derive(GraphQLObject)]
#[graphql(description = "A user")]
struct User {
    id: String,
    name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A new User")]
struct NewUser {
    name: String,
}

pub struct QueryRoot;

#[derive(Clone)]
pub struct Context {
    pub db: Addr<DbExecutor>
}

impl juniper::Context for Context {}

// graphql_object!(QueryRoot: Context |&self| {
//     field user(&executor, id: String) -> FieldResult<User> {
//         executor.context().db.send(GetUser{id: id.to_owned()})
//         .from_err()
//         .and_then(|res| {
//             match res {
//                 Ok(user) => Ok(HttpResponse::Ok().json(user)),
//                 Err(_) => Ok(HttpResponse::InternalServerError().into())
//             }
//         })
//         .responder()
//     }
// });
graphql_object!(QueryRoot: Context |&self| {
    field user(&executor, id: String) -> FieldResult<User> {
        Ok(User{
            id: "123".to_owned(),
            name: "Bob".to_owned(),
        })
    }
});

pub struct MutationRoot;

// graphql_object!(MutationRoot: Context |&self| {
//     field createUser(&executor, new_user: NewUser) -> FieldResult<User> {
//         executor.context().db.send(CreateUser{name: new_user.name.to_owned()})
//         .from_err()
//         .and_then(|res| {
//             match res {
//                 Ok(user) => Ok(HttpResponse::Ok().json(user)),
//                 Err(_) => Ok(HttpResponse::InternalServerError().into())
//             }
//         })
//         .responder()
//     }
// });
graphql_object!(MutationRoot: Context |&self| {
    field createUser(&executor, new_user: NewUser) -> FieldResult<User> {
        Ok(User{
            id: "123".to_owned(),
            name: "Bob".to_owned(),
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
