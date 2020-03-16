use juniper::FieldResult;

use super::context::GraphQLContext;

use crate::models::user::User;
use crate::models::project::Project;

pub mod user_qry;
pub mod project_qry;

// The root GraphQL query
pub struct QueryRoot;

// The root QueryRoot struct relies on GraphQLContext to provide
// the connection pool needed to execute actual Postgres queries.
#[juniper::object(Context = GraphQLContext)]
impl QueryRoot {
    // This annotation isn't really necessary, as Juniper would convert the
    // all_users function name into CamelCase. But I like to keep it explicit.

    // user
    #[graphql(name = "allUsers")]
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        user_qry::all_users(context)
    }

    #[graphql(name = "bannedUsers")]
    pub fn banned_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        user_qry::banned_users(context)
    }

    #[graphql(name = "notBannedUsers")]
    pub fn not_banned_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        user_qry::not_banned_users(context)
    }

    #[graphql(name = "getUserById")]
    pub fn get_user_by_id(context: &GraphQLContext, user_id: i32) -> FieldResult<Option<User>> {
        user_qry::get_user_by_id(context, user_id)
    }
    
    // project
    #[graphql(name = "allProjects")]
    pub fn all_projects(context: &GraphQLContext) -> FieldResult<Vec<Project>> {
        project_qry::all_projects(context)
    }

    #[graphql(name = "publishedProjects")]
    pub fn published_projects(context: &GraphQLContext) -> FieldResult<Vec<Project>> {
        project_qry::published_projects(context)
    }

    #[graphql(name = "notPublishedProjects")]
    pub fn not_published_projects(context: &GraphQLContext) -> FieldResult<Vec<Project>> {
        project_qry::not_published_projects(context)
    }

    #[graphql(name = "getProjectById")]
    pub fn get_project_by_id(context: &GraphQLContext, project_id: i32) -> FieldResult<Option<Project>> {
        project_qry::get_project_by_id(context, project_id)
    }
}
