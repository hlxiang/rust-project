use std::fs::File;
use std::io::Read;
// use toml::value::*;
use serde::{Deserialize, Serialize};

// todo: 可扩展性， 使用enum 或者 struct 来实现 json和toml文件的解析过程
// json文件解析
#[derive(Serialize, Deserialize, Debug)]
struct Args {
    development: Development,
}

#[derive(Serialize, Deserialize, Debug)]
struct Develop {
    address: String,
    port: String,
    workers: u64,
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

// toml文件解析
#[derive(Serialize, Deserialize, Debug)]
struct TomlArgs { 
    development: Develop,
    database: Database, // todo: #development.database, 需要名字的转换？参考letmeknow
}

fn main() {
    let mut json_file = File::open("./cfg_files/test1.json").unwrap();
    let mut buff = String::new();
    json_file.read_to_string(&mut buff).unwrap(); // todo:增加错误处理
    println!("1.1 read json file to buff: {:?}", buff);
    let readed_json: Args = serde_json::from_str(&buff).unwrap();
    println!("1.2 read json file: {:?}", readed_json);

    let mut toml_file = File::open("./cfg_files/test2.toml").unwrap();
    let mut toml_buff = String::new();
    toml_file.read_to_string(&mut toml_buff).unwrap();
    println!("2.1 read toml file to buff: {:?}", toml_buff);
    let readed_toml: TomlArgs = toml::from_str(&toml_buff).unwrap();
    println!("2.2 read toml file: {:?}", readed_toml); // toml_buff
}
