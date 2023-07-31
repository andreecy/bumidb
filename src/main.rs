use std::collections::HashMap;

use db::{Collection, Field, FieldType, Record};

mod db;

fn main() {
    match Collection::create(
        "users",
        vec![
            Field::new("name", FieldType::Text),
            Field::new("age", FieldType::Number),
        ],
    ) {
        Ok(_) => println!("collection has created"),
        Err(e) => println!("{}", e),
    }

    // match Collection::delete("users") {
    //     Ok(_) => println!("collection has deleted"),
    //     Err(e) => println!("{}", e),
    // }

    let mut data = HashMap::new();
    data.insert("name", "agus");
    data.insert("age", "23");

    Record::create("users", data).expect("cannot create record");
}
