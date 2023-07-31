use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum FieldType {
    Id,
    Text,
    Number,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub _type: FieldType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub name: String,
    pub schema: Vec<Field>,
}

impl Field {
    pub fn new(name: &str, _type: FieldType) -> Field {
        Field {
            name: name.to_owned(),
            _type,
        }
    }
}

const SCHEMA_PATH: &'static str = "db.schema";

impl Collection {
    /// Read schema file
    pub fn list() -> Result<Vec<Collection>, Box<dyn Error>> {
        let contents = fs::read_to_string(SCHEMA_PATH)?;
        let cols: Vec<Collection> = serde_json::from_str(&contents).expect("unable parse db");
        Ok(cols)
    }

    /// Write to schema file
    pub fn save(collections: &Vec<Collection>) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(collections).unwrap();
        let mut f = File::create(SCHEMA_PATH).expect("unable to create file");
        f.write_all(json.as_bytes()).expect("unable to write data");
        Ok(())
    }

    pub fn find(name: &str) -> Result<Collection, Box<dyn Error>> {
        let cols = Self::list()?;
        let mut iter = cols.into_iter();
        match iter.find(|x| x.name.eq(&name)) {
            Some(col) => {
                return Ok(col);
            }
            None => return Err(format!("collection {} not exists", &name))?,
        }
    }

    /// Create a collection
    pub fn create(name: &str, schema: Vec<Field>) -> Result<(), Box<dyn Error>> {
        // prepare data
        let mut sch = vec![Field {
            name: "_id".to_owned(),
            _type: FieldType::Id,
        }];
        sch.extend(schema);

        let col = Collection {
            name: name.to_string(),
            schema: sch,
        };

        let mut cols: Vec<Collection> = vec![];
        cols = match Self::list() {
            Ok(r) => r,
            Err(_) => {
                // db file not found, write a new file
                cols.push(col);
                Self::save(&cols)?;
                return Ok(());
            }
        };

        match Self::find(name) {
            Ok(_) => Err(format!(
                "collection {} already exists, consider using collection update",
                &name
            ))?,
            Err(_) => {
                // collection not found
                cols.push(col);
                Self::save(&cols)?;
                return Ok(());
            }
        }
    }

    /// Delete a collection
    pub fn delete(name: &str) -> Result<(), Box<dyn Error>> {
        // read existing schema
        let mut cols = Self::list()?;
        // db found, itterate to get existing collection to check wheater its exists
        let mut iter = cols.iter();
        match iter.position(|x| x.name.eq(name)) {
            Some(idx) => {
                cols.remove(idx);
                Self::save(&cols)?;
                return Ok(());
            }
            None => return Err(format!("collection {} not exists", name))?,
        }
    }
}
