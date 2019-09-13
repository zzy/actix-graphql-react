use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use futures::future::Future;

use bcrypt::{hash, verify};

use juniper::http::playground::playground_source;
use juniper::{http::GraphQLRequest, FieldResult};

use diesel::prelude::*;

use crate::db::{DbCon, DbPool};
use crate::gql_types::*;
use crate::jwt::{encode_jwt, verify_jwt};
use crate::models::*;

pub struct Context {
    db_con: DbCon,
    pub user_id: Option<i32>,
}
impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query:Context |&self|{
    field getProfile(&_executor) -> FieldResult<UserResponse>{
        use crate::schema::users::dsl::*;

        let user_id = match _executor.context().user_id {
            Some(user_id) => user_id,
            None => return Ok(UserResponse{ok:false, error:Some("Login required".to_string()), user:None}),
        };

        match users.find(user_id).first(&_executor.context().db_con) {
            Ok(user) => Ok(UserResponse{ok:true, error:None, user:Some(user)}),
            _ => Ok(UserResponse{ok:false, error:Some( "Not existing user".to_string() ), user:None}),
        }
    }
});

pub struct Mutation;

graphql_object!(Mutation:Context |&self|{
    field signIn(&_executor, email:String, password:String) -> FieldResult<TokenResponse>{

        use crate::schema::users;

        let user = users::dsl::users.filter(users::dsl::email.eq(email)).load::<User>(&_executor.context().db_con)?;

        let valid = verify(password, &user[0].password)?;

        if valid {
            let token = match encode_jwt(user[0].id, 30){
                Ok(t) => t,
                _ => return Ok(TokenResponse{token:None}),
            };
            Ok(TokenResponse{token:Some(token)})
        } else {
            Ok(TokenResponse{token:None})
        }
    }

    field signUp(&_executor, email:String, first_name:String, last_name:String, password:String, bio:Option<String>, avatar:Option<String>) -> FieldResult<UserResponse> {
        use crate::schema::users;

        let hashed = hash(&password, 10)?;

        let hashed_new_user = NewUser{
            email:email,
            first_name:first_name,
            last_name:last_name,
            password:hashed,
            bio:bio,
            avatar:avatar,
        };

        match diesel::insert_into(users::table)
            .values(&hashed_new_user)
            .get_result(&_executor.context().db_con){
                Ok(user) => Ok(UserResponse{ok:true,error:None, user:Some(user)}),
                _ => Ok(UserResponse{ok:false, error:None, user:None})
            }

    }

    field changePassword(&_executor, password:String) -> FieldResult<UserResponse>{
        use crate::schema::users;

        let user_id = match _executor.context().user_id{
            Some(user_id) => user_id,
            None => return Ok(UserResponse{ok:false, error:Some("Login required".to_string()), user:None}),
        };

        let hashed_new_password = match hash(&password, 10){
            Ok(pw)=>pw,
            _ => return Ok(UserResponse{ok:false, error: Some("Error hashing password".to_string()), user:None})
        };

        match diesel::update(users::dsl::users.find(user_id)).set(users::dsl::password.eq(hashed_new_password)).get_result::<User>(&_executor.context().db_con){
            Ok(user) => Ok(UserResponse{ok:true, error:None, user:Some(user)}),
            _ => Ok(UserResponse{ok:false, error: Some("Error".to_string()), user:None})
        }
    }

    field changeProfile(&_executor, bio:Option<String>, avatar:Option<String>) ->FieldResult<UserResponse>{
        use crate::schema::users;

        let user_id = match _executor.context().user_id {
            Some(user_id) => user_id,
            None => return Ok(UserResponse{ok:false, error:Some("Login required".to_string()), user:None}),
        };

        match diesel::update(users::dsl::users.find(user_id))
            .set(&UpdateUser {
                password:None,
                bio:bio,
                avatar:avatar,
            }).get_result::<User>(&_executor.context().db_con){
            Ok(user) => Ok(UserResponse{ok:true, error:None, user:Some(user)}),
            _ => Ok(UserResponse{ok:false, error: Some("Error".to_string()), user:None})
        }
    }
});

type Schema = juniper::RootNode<'static, Query, Mutation>;

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
        user_id: Some(1),
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
