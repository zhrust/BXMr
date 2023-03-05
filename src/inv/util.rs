#![allow(unused)]
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
//use std::io::prelude::*;
use std::io::{self, Write};
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;

use std::path::PathBuf;
use std::path::Path;

use std::collections::BTreeMap;

//use toml::de::{Deserializer, MapAccess, SeqAccess};
//use toml::de::{Deserializer, value::MapDeserializer, value::SeqDeserializer};
//use toml::Value;
use toml::value::{Value, Table};
//use toml::de::{Deserializer, MapAccess, SeqAccess, value::TableDeserializer, value::ArrayDeserializer};

use serde::{Serialize, Deserialize};
//use envy::Error;
use itertools::Itertools;
use dotenv::dotenv;

pub const ENV_YAML: &str = "BXMR_AIM_YAML";
pub const ENV_TOML: &str = "BXMR_TEMP_TOML";


pub fn toml2btmap(tfile:String) -> Option<BTreeMap<String, Vec<String>>> {
    let mut file = File::open(tfile).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let data: Value = toml::from_str(&contents).unwrap();

    let mut map = BTreeMap::new();
    for (key, value) in data.as_table().unwrap() {
        if let Some(array) = value.as_array() {
            let vec = array.iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect::<Vec<String>>();
            map.insert(key.clone(), vec.clone());
            //println!("insert: {}->{:#?}", key.clone(), vec.clone());
        }
    }

    //println!("{:#?}", map);

    Some(map)
}

pub const BXMC: &str = "abcdefghijklmnopqrstuvwxyz";
pub const MBCL: usize = 4; // code len.

pub fn generate_strings(length: usize, 
            prefix: String, 
            gbxm: &mut BTreeMap<String, Vec<String>>
        ) {
        if length == 0 {
            return;
        }
        for c in BXMC.chars() {
            let key = prefix.clone() + &c.to_string();
            //println!("{}", key);
            gbxm.insert(key.clone(), Vec::new());
            generate_strings(length - 1, key, gbxm);
        }
    }


pub fn init2(codelen:usize) -> Option<BTreeMap<String, Vec<String>>> {
    let mut gbxm = BTreeMap::new();
    //util::generate_strings(4, String::new(), &mut gbxm);
    //generate_strings(4, String::new(), &mut gbxm);
    generate_strings(codelen, String::new(), &mut gbxm);
    println!("\n\t gen. all BXM code as {} ", gbxm.len());

    Some(gbxm)
}


pub fn yaload(yfile:String) -> Vec<(String, String)> {
    let file = fs::File::open(yfile).expect("Failed to open file");
    let reader = BufReader::new(file);
    //let path = Path::new(&yfile);
    //let file = File::open(path)?;
    //let reader = BufReader::new(file);
    let lines = reader.lines().skip(10);

    let mut entries = Vec::new();
    
    for line in lines {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            //let (code,word) = (parts[1].to_string(), parts[0].to_string());
            entries.push((parts[1].to_string(), parts[0].to_string()));
            //println!("load:\t{}:{}",parts[1].to_string(), parts[0].to_string());
        }
    }

    entries
}

pub fn upd(key: &str, value: &str, gbxm: &mut BTreeMap<String, Vec<String>>) {
    match gbxm.get_mut(key) {
        Some(v) => {
            match v.iter().position(|x| x == value) {
                Some(_pos) => {
                    //println!("{} already exists in {:?}", value, key);
                    log::info!("{} already exists in {:?}", value, key);
                    //dbg!(format!("{} already exists in {:?}", value, key));
                },
                None => {
                    //v.push(value.to_owned());
                    v.insert(0, value.to_owned());
                    //println!("\n\t Updated {} in {:?} ", value, key);
                }
            }
        },
        None => {
            gbxm.insert(key.to_owned(), vec![value.to_owned()]);
            //println!("\n\t Added {} to {:?} ", value, key);
        }
    }
}


pub fn save2toml(code4btmap:BTreeMap<String, Vec<String>>, toml:String){

    // Convert BTreeMap to toml Value
    let toml_value = Value::try_from(code4btmap).unwrap();
    // Write toml Value to file
    let mut file = File::create(toml).unwrap();
    file.write_all(toml::to_string(&toml_value).unwrap().as_bytes()).unwrap();

}

/* 
pub fn upd(key: &str, value: &str, gbxm: &mut BTreeMap<String, Vec<String>>) {
    if let Some(v) = gbxm.get_mut(key) {
        if v.contains(&value.to_owned()) {
            //println!("{} already exists in {:?}", value, key);
            log::info!("{} already exists in {:?}", value, key);
            //dbg!(format!("{} already exists in {:?}", value, key));
        } else {
            //v.push(value.to_owned());
            v.insert(0, value.to_owned());
            //println!("\n\t Updated {} in {:?} ", value, key);
        }
    } else {
        gbxm.insert(key.to_owned(), vec![value.to_owned()]);
        //println!("\n\t Added {} to {:?} ", value, key);
    }
}
 */
pub fn print_gbxm_sorted(gbxm: &BTreeMap<String, Vec<String>>) {
    let mut sorted_keys: Vec<&String> = gbxm.keys().collect();
    sorted_keys.sort();

    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it

    for key in sorted_keys {
        let values = gbxm.get(key).unwrap();
        //println!("{} -> {:?}", key, values);
        writeln!(handle, 
            "{} -> {:?}", key, values
        ); // add `?` if you care about errors here
    }

}


pub fn ok_denv() -> Result<&'static Path, String> {
    // 获取应用程序的执行路径
    let exe_path = env::current_exe().map_err(|e| e.to_string())?;
    // 获取执行路径的父路径
    let exe_dir = exe_path.parent().ok_or_else(|| "Failed to get parent directory".to_string())?;
    // 构造 .env 文件路径
    let mut env_path = PathBuf::from(exe_dir);
    env_path.push(".env");

    // 如果 .env 文件不存在，创建一个空的
    if !env_path.exists() {
        let mut file = File::create(&env_path).map_err(|e| e.to_string())?;
        file.write_all(b"").map_err(|e| e.to_string())?;
    }
    // must leak for return ...
    Ok(Box::leak(env_path.into_boxed_path()))
}

/*
    //Ok(env_path.as_path())

    if env_path.exists() {
        Ok(env_path.as_path())
    } else {
        File::create(&env_path).map_err(|e| e.to_string())?;
        Ok(env_path.as_path())
    }

这是因为 env_path 是一个局部变量，它的生命周期与 ok_denv() 函数相同。如果将它的引用作为函数的返回值，会导致引用返回的变量已经被释放的问题。

为了解决这个问题，可以使用 Box<Path> 类型来将 Path 对象从堆上分配出来，并在函数返回时将其所有权移动到调用方。

*/


pub fn upd_denv(key: &str, val: &str) {
    match ok_denv() {
        Ok(path) => {
            let path_str = path.to_str().unwrap();
            let mut new_lines = String::new();
            let mut found_key = false;

            let file = File::open(path_str);
            let reader = BufReader::new(file.unwrap());
            for line in reader.lines() {
                if let Ok(l) = line {
                    if l.starts_with(&format!("{}=", key)) {
                        found_key = true;
                        new_lines.push_str(&format!("{}={} ", key, val));
                        println!("\n\t Updated .env item: {}={} ", key, val);
                    } else {
                        new_lines.push_str(&l);
                        new_lines.push('\n');
                    }
                }
            }

            // If the key doesn't exist in the .env file, add it
            if !found_key {
                new_lines.push_str(&format!("{}={} ", key, val));
                println!("\n\t New .env item, inserted: {}={} ", key, val);
            }

            let mut file = match File::create(path_str) {
                Ok(f) => f,
                Err(_) => {
                    println!("Failed to create .env file");
                    return;
                }
            };

            file.write_all(new_lines.as_bytes()).unwrap();
        },
        Err(e) => println!("{}", e),
    }
}


pub fn reload_denv(f2denv:&str){
    // 加载 .env 文件中的配置项
    //dotenv().ok();
    dotenv::from_path(&f2denv).ok();
    // 遍历当前进程中的所有环境变量，打印每个键值对
    //for (key, value) in std::env::vars() {
    //    println!("{}={}", key, value);
    //}
}


pub enum EnvResult {
    Success(String, String),
    Failure(String),
}

pub fn chk_denv(key: &str)-> EnvResult {
    match ok_denv() {
        Ok(f2denv) => {
            let f2denv = ok_denv().unwrap().to_str().unwrap();
            println!("\n\t load .env <-{} ", f2denv);
            dotenv::from_path(&f2denv).ok();
            //let val = std::env::var(key);

            match std::env::var(key) {
                Ok(val) => {
                    println!("\n\t got: {}={}", key, val);
                    EnvResult::Success(key.to_owned(), val)
                },
                Err(_) => {
                    println!("{} is not set in .env file", key);
                    EnvResult::Failure(format!("{} is not set in .env file", key))
                }
            }
            //if let Ok(env_val) = std::env::var(key){
            //    println!("{}={}", key, env_val);
            //}else{
            //    println!("{} is not set in .env file", key);
            //}
            //let f2denv = ok_denv().unwrap().to_str().unwrap();
            //list_denv(f2denv);
        },
        Err(e) => EnvResult::Failure(e.to_string()),
        //println!("{}", e),
    }
}

pub fn rmitem_denv(key: &str) {
    match ok_denv() {
        Ok(path) => {
            let path_str = path.to_str().unwrap();
            let file = File::open(path_str);
            match file {
                Ok(f) => {
                    let reader = BufReader::new(f);
                    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

                    let new_lines = lines
                            .iter()
                            .filter(|line| !line.starts_with(
                                &format!("{}=", key)))
                            .join(" ");

                    let mut file = File::create(path_str).unwrap();
                    file.write_all(new_lines.as_bytes()).unwrap();
                    println!("\n\t from .env removed item: {}", key);

                },
                Err(_) => println!("Failed to open .env file")
            }
        },
        Err(e) => println!("{}", e),
    }
}
