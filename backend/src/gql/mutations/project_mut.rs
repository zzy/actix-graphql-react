use diesel::pg::PgConnection;
use juniper::FieldResult;

use crate::gql::context::GraphQLContext;
use crate::models::{ Project, NewProject };
use crate::data::ProjectDao;

pub fn create_project(
    context: &GraphQLContext,
    new_project: NewProject
) -> FieldResult<Project> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    ProjectDao::create_project(conn, new_project)
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
