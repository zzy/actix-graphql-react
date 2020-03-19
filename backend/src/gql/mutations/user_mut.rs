use diesel::pg::PgConnection;
use juniper::FieldResult;

use crate::gql::context::GraphQLContext;
use crate::models::{ User, NewUser };
use crate::data::UserDao;

pub fn create_user(
    context: &GraphQLContext, 
    new_user: NewUser
) -> FieldResult<User> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    UserDao::create_user(conn, new_user)
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
