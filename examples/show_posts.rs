extern crate actix_web_juniper_react_apollo;
extern crate diesel;

use self::actix_web_juniper_react_apollo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use actix_web_juniper_react_apollo::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
