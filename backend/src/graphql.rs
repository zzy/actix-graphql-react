use super::context::GraphQLContext;
use diesel::pg::PgConnection;
use juniper::{FieldResult, RootNode};

use super::data::Users;
use super::models::{User, CreateUserInput};

// The root GraphQL query
pub struct QueryRoot;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
#[juniper::object(Context = GraphQLContext)]
impl QueryRoot {
    // This annotation isn't really necessary, as Juniper would convert the
    // all_users function name into CamelCase. But I like to keep it explicit.
    #[graphql(name = "allUsers")]
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        // TODO: pass the GraphQLContext into the querying functions
        // rather than a PgConnection (for brevity's sake)
        let conn: &PgConnection = &context.pool.get().unwrap();

        Users::all_users(conn)
    }

    #[graphql(name = "bannedUsers")]
    pub fn banned_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Users::banned_users(conn)
    }

    #[graphql(name = "notBannedUsers")]
    pub fn not_banned_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Users::not_banned_users(conn)
    }

    #[graphql(name = "getUserById")]
    pub fn get_user_by_id(
        context: &GraphQLContext,
        user_id: i32,
    ) -> FieldResult<Option<User>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Users::get_user_by_id(conn, user_id)
    }
}

// The root GraphQL mutation
pub struct MutationRoot;

#[juniper::object(Context = GraphQLContext)]
impl MutationRoot {
    #[graphql(name = "createUser")]
    pub fn create_user(
        context: &GraphQLContext,
        new_user: CreateUserInput,
    ) -> FieldResult<User> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Users::create_user(conn, new_user)
    }

    #[graphql(name = "markUserAsBanned")]
    pub fn mark_user_as_banned(
        context: &GraphQLContext, 
        user_id: i32
    ) -> FieldResult<User> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Users::mark_user_as_banned(conn, user_id)
    }

    #[graphql(name = "markUserAsNotBanned")]
    pub fn mark_user_as_not_banned(
        context: &GraphQLContext,
        user_id: i32,
    ) -> FieldResult<User> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Users::mark_user_as_not_banned(conn, user_id)
    }
}

// And finally the root schema that pulls the query and mutation together.
// Perhaps someday you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
