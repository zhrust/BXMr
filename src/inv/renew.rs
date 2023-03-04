//#![allow(unused)]
//use std::error::Error;
use std::fs;
//use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
//use std::io::{BufRead, BufReader};
//use std::path::Path;

//extern crate yaml_rust;
//use yaml_rust::{Yaml, YamlLoader};
//use serde::Deserialize;
//use serde::Serialize;

use crate::inv::util;

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
            println!("load:\t{}:{}",parts[1].to_string(), parts[0].to_string());
        }
    }

    entries
}

//pub fn load(yaml: String) {
pub fn load() {
    println!("src/inv/renew: {}", env!("CARGO_PKG_VERSION"));
    //log::debug!("src/inv/renew: from {}", yaml);

    //let cfg = util::settings();
    //let code_len = cfg["default"]["code_len"].as_integer().unwrap() as usize;
    //let p2rime = cfg["locsys"]["p2rime"].as_str().unwrap().to_string();
    //let bxm4mac = cfg["locsys"]["bxm4mac"].as_str().unwrap().to_string();
    //println!("_settings.toml\n\t{} {}/{}",util::MBCL,p2rime,bxm4mac);
    //// 在这里使用 code_len 和 loc_toml 进行您需要的操作
    //log::debug!("src/inv/renew: \n\tAIM-> {}/{}", p2rime,bxm4mac);
    ////let res = String::from_str(format!("{}/{}",p2rime,bxm4mac));
    /* 
    match util::chk_denv(util::ENV_YAML) {
        Ok((dkey, dval)) => {
            println!("Key is OK");
            println!("env hold:{}={}", dkey, dval);
        },
        Err(e) => print!("failed: {}", e),
    }
 */
    match util::chk_denv(util::ENV_YAML) {
        util::EnvResult::Success(dkey, dval) => {
            println!("Key is OK");
            println!("env hold:{}={}",dkey,dval);
            yaload(dval);
        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
    //let res = format!("{}/{}",p2rime,bxm4mac);
    //log::debug!("load()<- {}", res);
    //yaload(res);

}

/* 
#[derive(Debug, Deserialize)]
struct DictEntry {
    key: String,
    value: String,
}
 */


/* 
fn load_dict(yfile:String) -> Vec<DictEntry> {
    let file = File::open(yfile).unwrap();
    let reader = BufReader::new(file);
    let entries: Vec<DictEntry> = serde_yaml::from_reader(reader).unwrap();
    entries
}
 */