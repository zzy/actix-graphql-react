use diesel::pg::PgConnection;
use juniper::FieldResult;

use crate::graphql::context::GraphQLContext;

use crate::models::project::{ Project, ProjectInput };
use crate::data::project::ProjectDao;

// use super::MutationRoot;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
// #[juniper::object(Context = GraphQLContext)]
impl juniper::types::base::GraphQLType for MutationRoot {
    #[graphql(name = "createProject")]
    pub fn create_project(
        context: &GraphQLContext,
        project_input: ProjectInput,
    ) -> FieldResult<Project> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        ProjectDao::create_project(conn, project_input)
    }

    #[graphql(name = "markProjectAsPublished")]
    pub fn mark_project_as_published(
        context: &GraphQLContext, 
        project_id: i32
    ) -> FieldResult<Project> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        ProjectDao::mark_project_as_published(conn, project_id)
    }

    #[graphql(name = "markProjectAsNotPublished")]
    pub fn mark_project_as_not_published(
        context: &GraphQLContext,
        project_id: i32,
    ) -> FieldResult<Project> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        ProjectDao::mark_project_as_not_published(conn, project_id)
    }
}
