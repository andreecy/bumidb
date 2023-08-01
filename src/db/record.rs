use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::{collections::HashMap, error::Error};

use uuid::Uuid;

use super::{Collection, Field};

type RecordData = HashMap<String, String>;

pub struct Record {
    data: RecordData,
}

fn parse_string(string: String, separator: char) -> Vec<String> {
    let s = string.replace("\n", "");
    s.split(separator).map(|x| x.into()).collect()
}

impl Record {
    pub fn find(collection: &str, id: &str) -> Result<(), Box<dyn Error>> {
        let f = File::open(collection)?;
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        let mut founds: Vec<String> = vec![];

        while reader.read_line(&mut line).unwrap() > 0 {
            if line.matches(id).count() > 0 {
                founds.push(line.to_owned());
            }
            line.clear();
        }

        let asd: Vec<Vec<String>> = founds.into_iter().map(|x| parse_string(x, ',')).collect();
        println!("{:?}", asd);
        Ok(())
    }

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
                        "_id" => Uuid::new_v4().to_string(),
                        _ => match data.get(&f) {
                            Some(v) => v.to_owned(),
                            None => "".to_owned(),
                        },
                    })
                    .collect::<Vec<String>>()
                    .join(",");

                println!("{}", value);

                let mut f = OpenOptions::new().append(true).open(collection)?;
                f.write(value.as_bytes())?;
                f.write(b"\n")?;
                Ok(())
            }
            Err(e) => return Err(e),
        }
    }
}
