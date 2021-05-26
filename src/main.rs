use std::collections::HashMap;
use std::env::args;
use std::env::Args;
use std::fs;
use std::io::Error;
use std::iter::Skip;
use std::path::PathBuf;

fn main() {
    let mut arguments: Skip<Args> = args().skip(1);
    let key: String = arguments.next().expect("Key was not there");
    let value: String = arguments.next().expect("Value was not there");

    println!("The key is '{}' and the value is '{}'", key, value);

    let mut database = Database::new().expect("Database::new() failed");
    database.insert(key.clone(), value.clone());
    database.insert(key.to_uppercase(), value.to_uppercase());
    database.insert(format!("{}2", key), format!("{}2", value));
    println!("{:?}", database.1);



    struct Database(PathBuf, HashMap<String, String>);

    impl Database {
        fn new() -> Result<Database, Error> {
            let path: PathBuf = PathBuf::from(r"C:\Users\Catherine\Documents\Projects\rust-intro\kv.db");
            let mut map = HashMap::new();
            let contents = fs::read_to_string("kv.db")?;
            for line in contents.lines() {
                let (key, value) = line.split_once('\t').expect("Corrupt database");
                map.insert(key.to_owned(), value.to_owned());
            }

            Ok(Database(path, map))
        }

        fn insert(&mut self, key: String, value: String) {
            self.1.insert(key, value);
        }

        // fn delete(self) {}
    }
}
