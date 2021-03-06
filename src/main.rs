use std::collections::HashMap;
use std::env::args;
use std::env::Args;
use std::fs;
use std::io::Result;
use std::iter::Skip;
use std::path::PathBuf;

fn main() {
    let mut arguments: Skip<Args> = args().skip(1);
    let key: String = arguments.next().expect("Key was not there");
    let value: String = arguments.next().expect("Value was not there");

    println!("The new value of '{}' is now '{}'", key, value);

    let path: PathBuf = PathBuf::from(r"C:\Users\Catherine\Documents\Projects\rust-intro\kv.db");
    let mut database = Database::new(path).expect("Database::new() failed");
    database.insert(key.clone(), value.clone());
    database.flush();
}


struct Database(PathBuf, HashMap<String, String>);

impl Database {
    fn new(path: PathBuf) -> Result<Database> {
        let mut map = HashMap::new();
        let contents = fs::read_to_string(&path)?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database(path, map))
    }

    fn insert(&mut self, key: String, value: String) {
        self.1.insert(key, value);
    }

    fn flush(self) -> Result<()> {
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let _ = do_flush(self);
    }
}

fn do_flush(database: &Database) -> Result<()> {
    println!("do_flush called");
    let mut contents = String::new();
    for (key, value) in &database.1 {
        contents.push_str(&key);
        contents.push('\t');
        contents.push_str(&value);
        contents.push('\n');
    }
    fs::write(&database.0, contents)
}
