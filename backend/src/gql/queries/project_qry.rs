use diesel::pg::PgConnection;
use juniper::FieldResult;

use crate::gql::context::GraphQLContext;
use crate::models::Project;
use crate::data::ProjectDao;

pub fn all_projects(
    context: &GraphQLContext
) -> FieldResult<Vec<Project>> {
    // TODO: pass the GraphQLContext into the querying functions
    // rather than a PgConnection (for brevity's sake)
    let conn: &PgConnection = &context.pool.get().unwrap();

    ProjectDao::all_projects(conn)
}

pub fn get_projects_by_user(
    context: &GraphQLContext,
    project_user_id: i32
) -> FieldResult<Vec<Project>> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    ProjectDao::get_projects_by_user(conn, project_user_id)
}

pub fn get_project_by_id(
    context: &GraphQLContext,
    project_id: i32
) -> FieldResult<Option<Project>> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    ProjectDao::get_project_by_id(conn, project_id)
}

pub fn published_projects(
    context: &GraphQLContext
) -> FieldResult<Vec<Project>> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    ProjectDao::published_projects(conn)
}

pub fn not_published_projects(
    context: &GraphQLContext
) -> FieldResult<Vec<Project>> {
    let conn: &PgConnection = &context.pool.get().unwrap();

    ProjectDao::not_published_projects(conn)
}
