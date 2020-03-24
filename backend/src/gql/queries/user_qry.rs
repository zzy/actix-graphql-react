use diesel::pg::PgConnection;
use juniper::FieldResult;

use crate::gql::context::GraphQLContext;
use crate::data::UserDao;
use crate::models::User;

pub fn all_users(
    context: &GraphQLContext
) -> FieldResult<Vec<User>> {
    // TODO: pass the GraphQLContext into the querying functions
    // rather than a PgConnection (for brevity's sake)
    let conn: &PgConnection = &context.pool.get().unwrap();

    UserDao::all_users(conn)
}

pub fn get_user_by_id(
    context: &GraphQLContext,
    user_id: i32
) -> FieldResult<Option<User>> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    UserDao::get_user_by_id(conn, user_id)
}

pub fn get_user_by_email_or_username(
    context: &GraphQLContext,
    email_or_username: String,
) -> FieldResult<Option<User>> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    UserDao::get_user_by_email_or_username(conn, email_or_username)
}

pub fn banned_users(
    context: &GraphQLContext
) -> FieldResult<Vec<User>> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    UserDao::banned_users(conn)
}

pub fn not_banned_users(
    context: &GraphQLContext
) -> FieldResult<Vec<User>> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    UserDao::not_banned_users(conn)
}
