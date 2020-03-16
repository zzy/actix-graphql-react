use diesel::pg::PgConnection;
use juniper::FieldResult;

use crate::graphql::context::GraphQLContext;
use crate::models::project::{Project, ProjectInput};
use crate::data::project::ProjectDao;

pub fn create_project(
    context: &GraphQLContext,
    project_input: ProjectInput
) -> FieldResult<Project> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    ProjectDao::create_project(conn, project_input)
}

pub fn mark_project_as_published(
    context: &GraphQLContext,
    project_id: i32
) -> FieldResult<Project> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    ProjectDao::mark_project_as_published(conn, project_id)
}

pub fn mark_project_as_not_published(
    context: &GraphQLContext,
    project_id: i32
) -> FieldResult<Project> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    ProjectDao::mark_project_as_not_published(conn, project_id)
}
