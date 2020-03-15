use diesel::pg::PgConnection;
use juniper::FieldResult;

use super::context::GraphQLContext;

use crate::models::user::User;
use crate::data::user::UserDao;

use crate::models::project::Project;
use crate::data::project::ProjectDao;

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

        UserDao::all_users(conn)
    }

    #[graphql(name = "bannedUsers")]
    pub fn banned_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        UserDao::banned_users(conn)
    }

    #[graphql(name = "notBannedUsers")]
    pub fn not_banned_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        UserDao::not_banned_users(conn)
    }

    #[graphql(name = "getUserById")]
    pub fn get_user_by_id(
        context: &GraphQLContext,
        user_id: i32,
    ) -> FieldResult<Option<User>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        UserDao::get_user_by_id(conn, user_id)
    }

        // This annotation isn't really necessary, as Juniper would convert the
    // all_users function name into CamelCase. But I like to keep it explicit.
    #[graphql(name = "allProjects")]
    pub fn all_projects(context: &GraphQLContext) -> FieldResult<Vec<Project>> {
        // TODO: pass the GraphQLContext into the querying functions
        // rather than a PgConnection (for brevity's sake)
        let conn: &PgConnection = &context.pool.get().unwrap();

        ProjectDao::all_projects(conn)
    }

    #[graphql(name = "publishedProjects")]
    pub fn published_projects(context: &GraphQLContext) -> FieldResult<Vec<Project>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        ProjectDao::published_projects(conn)
    }

    #[graphql(name = "notPublishedProjects")]
    pub fn not_published_projects(context: &GraphQLContext) -> FieldResult<Vec<Project>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        ProjectDao::not_published_projects(conn)
    }

    #[graphql(name = "getProjectById")]
    pub fn get_project_by_id(
        context: &GraphQLContext,
        project_id: i32,
    ) -> FieldResult<Option<Project>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        ProjectDao::get_project_by_id(conn, project_id)
    }
}
