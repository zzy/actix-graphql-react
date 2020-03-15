use diesel::pg::PgConnection;
use juniper::FieldResult;

use super::context::GraphQLContext;

use crate::models::project::Project;
use crate::data::project::ProjectDao;

// use super::QueryRoot;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
// #[juniper::object(Context = GraphQLContext)]
impl juniper::types::base::GraphQLType for QueryRoot {
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
