use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

use serde_json as json;

use components::*;

#[derive(Debug, Default)]
pub struct BlueprintManager {
    blueprints: HashMap<String, Blueprint>,
}

impl BlueprintManager {
    pub fn load<P: AsRef<Path>>(json_path: P) -> io::Result<BlueprintManager> {
        let mut blueprints_file = fs::File::open(json_path)?;
        let mut blueprints_str = String::new();
        blueprints_file.read_to_string(&mut blueprints_str)?;

        let json_data = json::from_str::<HashMap<String, Blueprint>>(&blueprints_str);
        match json_data {
            Ok(json_data) => {
                Ok(BlueprintManager {
                    blueprints: json_data,
                })
            },
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Blueprint> {
        self.blueprints.get(name)
    }
}
