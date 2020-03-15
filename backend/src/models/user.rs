#[allow(unused_imports)]

use juniper::GraphQLInputObject;
use chrono::*;

use crate::schema::users;

// The core data type undergirding the GraphQL interface
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub passord: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub banned: bool,
}

// applying #[derive(juniper::GraphQLObject)] to the User struct above
#[juniper::object(description = "Users in the platform")]
impl User {
    #[graphql(name = "id")]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[graphql(name = "email")]
    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    #[graphql(name = "username")]
    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    #[graphql(name = "created_at")]
    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.created_at, Utc)
    }

    #[graphql(name = "updated_at")]
    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.updated_at, Utc)
    }

    #[graphql(name = "banned")]
    pub fn banned(&self) -> bool {
        self.banned
    }
}

// Used to create new user
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub banned: &'a bool,
}

// The GraphQL input object for creating user
#[derive(GraphQLInputObject)]
pub struct UserInput {
    pub email: String,
    pub username: String,
    pub password: String,
    pub banned: Option<bool>,
}
