use uuid::Uuid;

fn main() {
    let my_uuid = Uuid::new_v4();
    println!("{}", my_uuid);

    let my_uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
    println!("{}", my_uuid.to_urn());
}
