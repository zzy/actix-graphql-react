use chrono::*;

use crate::schema::projects;

// The core data type undergirding the GraphQL interface
#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub user_id: i32,
    pub subject: String,
    pub website: String,
    pub source_code: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub published: bool,
}

// applying #[derive(juniper::GraphQLObject)] to the project struct above
#[juniper::object(description = "A project of a user")]
impl Project {
    #[graphql(name = "id")]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[graphql(name = "user_id")]
    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    #[graphql(name = "subject")]
    pub fn subject(&self) -> &str {
        self.subject.as_str()
    }

    #[graphql(name = "website")]
    pub fn website(&self) -> &str {
        self.website.as_str()
    }

    #[graphql(name = "source_code")]
    pub fn source_code(&self) -> &str {
        self.source_code.as_str()
    }

    #[graphql(name = "created_at")]
    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.created_at, Utc)
    }

    #[graphql(name = "updated_at")]
    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.updated_at, Utc)
    }

    #[graphql(name = "published")]
    pub fn published(&self) -> bool {
        self.published
    }
}

// It is super convenient that Rust allows us to use the same structs
// for GraphQL input objects as well as Diesel insertable objects.
#[derive(GraphQLInputObject, Insertable)]
#[table_name = "projects"]
pub struct NewProject {
    pub user_id: i32,
    pub subject: String,
    pub website: String,
    pub source_code: String,
    // pub published: bool,
}
