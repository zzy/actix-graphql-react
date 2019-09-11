use super::schema::*;
use chrono::prelude::*;

#[derive(Queryable)]
pub struct Cat {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "cats"]
pub struct NewCat {
    pub name: String,
}

#[derive(Queryable, GraphQLObject, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub fist_name: String,
    pub last_name: String,
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
