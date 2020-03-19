use juniper::FieldResult;

use super::context::GraphQLContext;

use crate::models::{ User, NewUser, Project, NewProject };

pub mod user_mut;
pub mod project_mut;

// The root GraphQL mutation
pub struct MutationRoot;

// The root MutationRoot struct relies on GraphQLContext to provide
// the connection pool needed to execute actual Postgres queries.
#[juniper::object(Context = GraphQLContext)]
impl MutationRoot {
    // This annotation isn't really necessary, as Juniper would convert the
    // all_users function name into CamelCase. But I like to keep it explicit.

    // user
    #[graphql(name = "createUser")]
    pub fn create_user(context: &GraphQLContext, new_user: NewUser) -> FieldResult<User> {
        user_mut::create_user(context, new_user)
    }

    #[graphql(name = "markUserAsBanned")]
    pub fn mark_user_as_banned(context: &GraphQLContext, user_id: i32) -> FieldResult<User> {
        user_mut::mark_user_as_banned(context, user_id)
    }

    #[graphql(name = "markUserAsNotBanned")]
    pub fn mark_user_as_not_banned(context: &GraphQLContext, user_id: i32) -> FieldResult<User> {
        user_mut::mark_user_as_not_banned(context, user_id)
    }

    // project
    #[graphql(name = "createProject")]
    pub fn create_project(context: &GraphQLContext, new_project: NewProject) -> FieldResult<Project> {
        project_mut::create_project(context, new_project)
    }

    #[graphql(name = "markProjectAsPublished")]
    pub fn mark_project_as_published(context: &GraphQLContext, project_id: i32) -> FieldResult<Project> {
        project_mut::mark_project_as_published(context, project_id)
    }

    #[graphql(name = "markProjectAsNotPublished")]
    pub fn mark_project_as_not_published(context: &GraphQLContext, project_id: i32) -> FieldResult<Project> {
        project_mut::mark_project_as_not_published(context, project_id)
    }
}
