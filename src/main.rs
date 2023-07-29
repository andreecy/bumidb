use std::{
    error::Error,
    fs::{self, File},
    io::Write,
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

/// Read schema file
fn schema_read() -> Result<Vec<Collection>, Box<dyn Error>> {
    let contents = fs::read_to_string(SCHEMA_PATH)?;
    let cols: Vec<Collection> = serde_json::from_str(&contents).expect("unable parse db");
    Ok(cols)
}

/// Write to schema file
fn schema_write(collections: &Vec<Collection>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string(collections).unwrap();
    let mut f = File::create(SCHEMA_PATH).expect("unable to create file");
    f.write_all(json.as_bytes()).expect("unable to write data");
    Ok(())
}

/// Create a collection
fn collection_create(name: String, schema: Vec<Field>) -> Result<(), Box<dyn Error>> {
    // prepare data
    let mut sch = vec![Field {
        name: "_id".to_owned(),
        _type: FieldType::Id,
    }];
    sch.extend(schema);

    let col = Collection {
        name: name.clone(),
        schema: sch,
    };

    let mut cols: Vec<Collection> = vec![];

    // read existing schema
    cols = match schema_read() {
        Ok(r) => r,
        Err(_e) => {
            // no schema found then write new file
            cols.push(col);
            schema_write(&cols).expect("cannot write schema");
            return Ok(());
        }
    };

    // db found, itterate to get existing collection to check wheater its exists
    let mut iter = cols.iter();

    match iter.find(|x| x.name.eq(&name)) {
        Some(_) => Err(format!(
            "collection {} already exists, consider using collection update",
            &name
        ))?,
        None => {
            // collection not exists, add to list of cols then update file
            cols.push(col);
            schema_write(&cols).expect("cannot write schema");
            Ok(())
        }
    }
}

/// Delete a collection
fn collection_delete(name: String) -> Result<(), Box<dyn Error>> {
    // read existing schema
    let mut cols = schema_read()?;
    // db found, itterate to get existing collection to check wheater its exists
    let mut iter = cols.iter();
    match iter.position(|x| x.name.eq(&name)) {
        Some(idx) => {
            cols.remove(idx);
            schema_write(&cols)?;
            return Ok(());
        }
        None => return Err(format!("collection {} not exists", &name))?,
    }
}

fn main() {
    match collection_create(
        "admin".to_owned(),
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
    ) {
        Ok(_) => println!("collection admin has created"),
        Err(e) => println!("{}", e),
    }

    collection_delete("admin".to_owned()).expect("cannot delete collection");

    // println!("{:?}", user);
}
