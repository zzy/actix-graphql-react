use diesel::pg::PgConnection;
use juniper::FieldResult;

use crate::graphql::context::GraphQLContext;

use crate::data::user::UserDao;
use crate::models::user::{User, UserInput};

pub fn create_user(
    context: &GraphQLContext, 
    user_input: UserInput
) -> FieldResult<User> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    UserDao::create_user(conn, user_input)
}

pub fn mark_user_as_banned(
    context: &GraphQLContext, 
    user_id: i32
) -> FieldResult<User> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    UserDao::mark_user_as_banned(conn, user_id)
}

pub fn mark_user_as_not_banned(
    context: &GraphQLContext, 
    user_id: i32
) -> FieldResult<User> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    UserDao::mark_user_as_not_banned(conn, user_id)
}
