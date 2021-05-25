use std::fs;
use std::env::args;
use std::env::Args;
use std::iter::Skip;

fn main() -> std::io::Result<()> {
    let mut arguments: Skip<Args> = args().skip(1);
    let key: String = arguments.next().expect("Key was not there");
    let value: String = arguments.next().expect("Value was not there");

    println!("The key is '{}' and the value is '{}'", key, value);

    let contents: String = format!("{}\t{}\n", key, value);
    fs::write("kv.db", contents)
}
