use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum FieldType {
    Id,
    Text,
    Number,
}

#[derive(Serialize, Deserialize, Debug)]
struct Field {
    name: String,
    _type: FieldType,
}

#[derive(Serialize, Deserialize, Debug)]
struct Collection {
    name: String,
    schema: Vec<Field>,
}

const SCHEMA_PATH: &'static str = "db.schema";

fn db_read() -> Result<Vec<Collection>, std::io::Error> {
    let f = File::open(SCHEMA_PATH)?;
    let reader = BufReader::new(f);

    let mut cols: Vec<Collection> = Vec::new();

    for line in reader.lines() {
        let l = line.expect("eror read line");
        let col: Collection = serde_json::from_str(&l).expect("unable parse db");
        cols.push(col)
    }

    Ok(cols)
}

/// Create a collection
fn collection_create(name: String, schema: Vec<Field>) -> Result<(), Box<dyn Error>> {
    let cols = db_read()?;
    println!("{:?}", cols);

    let mut iter = cols.iter();

    match iter.find(|&x| x.name.eq(&name)) {
        Some(_) => Err(format!("collection {} already exists", &name).to_owned())?,
        None => {
            let mut sch = vec![Field {
                name: "_id".to_owned(),
                _type: FieldType::Id,
            }];

            sch.extend(schema);

            let col = Collection {
                name: name.clone(),
                schema: sch,
            };
            let json = serde_json::to_string(&col).unwrap();

            let mut f = File::create(SCHEMA_PATH).expect("unable to create file");
            f.write_all(json.as_bytes()).expect("unable to write data");
            println!("creating collection {}", name);

            Ok(())
        }
    }
}

fn main() {
    collection_create(
        "user".to_owned(),
        vec![
            Field {
                name: "name".to_owned(),
                _type: FieldType::Text,
            },
            Field {
                name: "age".to_owned(),
                _type: FieldType::Number,
            },
        ],
    )
    .expect("cannot create collection");

    // println!("{:?}", user);
}
