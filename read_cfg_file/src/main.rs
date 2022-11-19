use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Args {
    development: Development,
}

#[derive(Serialize, Deserialize, Debug)]
struct Development {
    address: String,
    port: String,
    workers: u64,
    database: Database,
}
#[derive(Serialize, Deserialize, Debug)]
struct Database {
    adapter: String,
    db_name: String,
    pool: u8,
}

fn main() {
    let mut json_file = File::open("./cfg_files/test1.json").unwrap();
    let mut buff = String::new();
    json_file.read_to_string(&mut buff).unwrap();

    let readed_json: Args = serde_json::from_str(&buff).unwrap();
    println!("read json file: {:?}", readed_json);
}
