use diesel::pg::PgConnection;
use juniper::FieldResult;

use crate::data::user::UserDao;
use crate::models::user::{User, UserInput};
use super::context::GraphQLContext;

// The root GraphQL mutation
pub struct MutationRoot;

#[juniper::object(Context = GraphQLContext)]
impl MutationRoot {
    #[graphql(name = "createUser")]
    pub fn create_user(
        context: &GraphQLContext,
        user_input: UserInput,
    ) -> FieldResult<User> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        UserDao::create_user(conn, user_input)
    }

    #[graphql(name = "markUserAsBanned")]
    pub fn mark_user_as_banned(
        context: &GraphQLContext, 
        user_id: i32
    ) -> FieldResult<User> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        UserDao::mark_user_as_banned(conn, user_id)
    }

    #[graphql(name = "markUserAsNotBanned")]
    pub fn mark_user_as_not_banned(
        context: &GraphQLContext,
        user_id: i32,
    ) -> FieldResult<User> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        UserDao::mark_user_as_not_banned(conn, user_id)
    }
}
