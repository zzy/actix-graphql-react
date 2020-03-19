use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::schema::projects::dsl::*;
use crate::models::{ Project, NewProject };
use super::utils::graphql_translate;

// This struct is basically a query manager. All the methods that it
// provides are static, making it a convenient abstraction for
// interacting with the database.
pub struct ProjectDao;

// Note that all the function names here map directly onto the function names
// associated with the QueryRoot and MutationRoot structs.
// This is NOT necessary but I personally prefer it.
impl ProjectDao {
    pub fn all_projects(conn: &PgConnection) -> FieldResult<Vec<Project>> {
        let res = projects.load::<Project>(conn);

        graphql_translate(res)
    }

    pub fn get_projects_by_user(
        conn: &PgConnection,
        project_user_id: i32
    ) -> FieldResult<Vec<Project>> {
        let res = projects.filter(user_id.eq(project_user_id)).load::<Project>(conn);

        graphql_translate(res)
    }
    
    pub fn get_project_by_id(
        conn: &PgConnection,
        project_id: i32
    ) -> FieldResult<Option<Project>> {
        match projects.find(project_id).get_result::<Project>(conn) {
            Ok(project) => Ok(Some(project)),
            Err(e) => match e {
                // Without this translation, GraphQL will return an error rather
                // than the more semantically sound JSON null if no Project is found.
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                _ => FieldResult::Err(FieldError::from(e)),
            },
        }
    }

    pub fn create_project(
        conn: &PgConnection,
        new_project: NewProject,
    ) -> FieldResult<Project> {
        use crate::schema::projects;

        let res = diesel::insert_into(projects::table)
            .values(&new_project)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn published_projects(conn: &PgConnection) -> FieldResult<Vec<Project>> {
        let res = projects.filter(published.eq(true)).load::<Project>(conn);

        graphql_translate(res)
    }

    pub fn not_published_projects(conn: &PgConnection) -> FieldResult<Vec<Project>> {
        let res = projects.filter(published.eq(false)).load::<Project>(conn);

        graphql_translate(res)
    }

    pub fn mark_project_as_published(
        conn: &PgConnection, 
        project_id: i32
    ) -> FieldResult<Project> {
        mark_project_as(conn, project_id, true)
    }

    pub fn mark_project_as_not_published(
        conn: &PgConnection,
        project_id: i32
    ) -> FieldResult<Project> {
        mark_project_as(conn, project_id, false)
    }
}

// This helper function ensures that projects don't mark Project as published that are already published
// (or not published that are already not published).
fn mark_project_as(
    conn: &PgConnection, 
    project_id: i32, 
    is_published: bool
) -> FieldResult<Project> {
    let res = projects.find(project_id).get_result::<Project>(conn);

    // Poor man's Ternary operator for error output text
    let msg = if is_published { "published" } else { "not published" };

    match res {
        Ok(project) => {
            if project.published == is_published {
                let err = FieldError::new(
                    format!("Project already marked as {}", msg),
                    // TODO: better error output
                    graphql_value!({ "cannot_update": "confict"}),
                );
                FieldResult::Err(err)
            }
            else {
                let res = diesel::update(projects.find(project_id))
                    .set(published.eq(is_published))
                    .get_result::<Project>(conn);
                graphql_translate(res)
            }
        }
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}
