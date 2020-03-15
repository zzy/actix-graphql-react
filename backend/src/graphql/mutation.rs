use diesel::pg::PgConnection;
use juniper::FieldResult;

use super::context::GraphQLContext;

use crate::models::user::{ User, UserInput };
use crate::data::user::UserDao;

use crate::models::project::{ Project, ProjectInput };
use crate::data::project::ProjectDao;

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
