use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let graphql_port: i16 = env::var("GRAPHQL_PORT")
        .unwrap_or_else(|_| String::from("80"))
        .parse()
        .expect("GRAPHQL_PORT must be a number");

    println!("0.0.0.0:{}", graphql_port);
}
