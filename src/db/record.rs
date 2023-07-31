use std::fs::File;
use std::io::Write;
use std::{collections::HashMap, error::Error};

use super::{Collection, Field};

type RecordData = HashMap<String, String>;

pub struct Record {
    data: RecordData,
}

impl Record {
    pub fn create(collection: &str, data: HashMap<&str, &str>) -> Result<(), Box<dyn Error>> {
        match Collection::find(collection) {
            Ok(col) => {
                let fields = col
                    .schema
                    .into_iter()
                    .map(|x| x.name)
                    .collect::<Vec<String>>();

                let header = fields.join(",");

                println!("{}", header);

                let data = data
                    .into_iter()
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                    .collect::<HashMap<String, String>>();

                let value = fields
                    .into_iter()
                    .map(|f| match f.as_str() {
                        "_id" => "randomId".to_owned(),
                        _ => match data.get(&f) {
                            Some(v) => v.to_owned(),
                            None => "".to_owned(),
                        },
                    })
                    .collect::<Vec<String>>()
                    .join(",");

                println!("{}", value);

                let mut f = File::create(collection).expect("unable to create file");
                f.write_all(value.as_bytes()).expect("unable to write data");
                Ok(())
            }
            Err(e) => return Err(e),
        }
    }
}
