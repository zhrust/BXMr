//! Utility functions for BXMr
//! BXMr 工具函数模块
//! 
//! This module contains utility functions, some reserved for future use.
//! 此模块包含工具函数，部分保留供将来使用。

#![allow(dead_code)]

use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;

use std::path::PathBuf;
use std::path::Path;

use std::collections::BTreeMap;

use toml::value::Value;

use itertools::Itertools;
use indicatif::ProgressBar;

/// Environment variable name for YAML path
/// 环境变量名：YAML 路径
pub const ENV_YAML: &str = "BXMR_AIM_YAML";

#[allow(dead_code)]
/// Environment variable name for TOML path (reserved for future use)
/// 环境变量名：TOML 路径（保留供将来使用）
pub const ENV_TOML: &str = "BXMR_TEMP_TOML";

/// BXM code character set / 表形码字符集
pub const BXMC: &str = "abcdefghijklmnopqrstuvwxyz";

/// Environment variable name for CSV index path
/// 环境变量名：CSV 索引文件路径
pub const ENV_IDX_CSV: &str = "BXMR_IDX_CSV";

#[allow(dead_code)]
/// Maximum BXM code length / 表形码最大长度
pub const MBCL: usize = 4;

pub const RIME_HEAD: &str = r#"
# Rime dictionary
# encoding: utf-8

---
name: bxm4zq2mac
version: "v.23.03.05.1642"
sort: original
...

"#;

pub const H_HELP: &str = r#"
-----------------------------------------
BXMr Commands:
help   Print this message or the help of the given subcommand(s)
cfg    point where is u rIME base *.dict.yaml
env    check bind ENV setting, work with command:cfg
ver    echo current BXMr version info.
gen    re-generating .yaml -> ~/Library/Rime/[U BXM].yaml,config by command: cfg
seek   base code SEEK word is exist?
find   base word FIND code is exist?
upd    aaa 叒 <~ base code word UPGRADE the define in BXM
dele   aaa 叒 ~> base code word DELET the define from BXM
ahead  aaa 叒 => base code word UP the word define 1st in BXM
atail  aaa 叒 => base code word DOWN the word define Last in BXM
idx    re-build char index from current BXM table (to .csv)
apd    (Interative) append new word with auto-generated code

-----------------------------------------
BXMr Usage:
0: must setup .env for all Commands;
$ bxmr
BXMr> cfg yaml path/2/u/local/bxm4zq2mac.dict.yaml
        ~ point u rIME-Squirrel usage .yaml
>>> daily usage flow loop:
$ bxmr 
BXMr> ?
... print this help

BXMr> seek aaa
.. seek the code is exist?
BXMr> upd aaa 叒
... if not exist, u can append it:
    $ bxmr upd aaa 叒
BXMr> find 叒
... or find the word's code is exist? ~> find 字词
BXMr> gen
... if enough now, must export to .yaml:

... and other BXM code adjust:
BXMr> dele   aaa 叒
... ~> base code word DELET the define from BXM
BXMr> ahead  aaa 叒
... => base code word UP the word define 1st in BXM

... if want research BXMr
BXMr> env
... check bind ENV setting, work with command:cfg
BXMr> ver
... echo current BXMr version info.

... if want exit BXMr
BXMr> CTRL-D | CTRL-C

at last, always need usage rIME's re-deploy menu, 
    for load new code-table .yaml,
    so enjoy your new BXM now ;-)
    "#;

pub const H_MORE: &str = r#"
-----------------------------------------
CTRL-D | CTRL-C can exit BXMr
all BXMr command, ask> ?|h|help
    "#;

pub const H_CFG: &str = r#"
-----------------------------------------
usage as:
BXMr> cfg yaml path/2/u/local/bxm4zq2mac.dict.yaml
~ point u rIME-Squirrel usage .yaml
e.g: for me
cfg yaml /Users/zoomq/Library/Rime/bxm4zq2mac.dict.yaml

all BXMr command, call ?|h|help
    "#;

#[allow(dead_code)]
pub const H_ENV: &str = r#"
-----------------------------------------
usage as:
BXMr> env 
~ will print config info.

all BXMr command, call ?|h|help
    "#;

#[allow(dead_code)]
pub const H_SEEK: &str = r#"
-----------------------------------------
usage as:
BXMr> seek aaa 
~ base code SEEK word is exist?

all BXMr command, call ?|h|help
    "#;

#[allow(dead_code)]
pub const H_FIND: &str = r#"
-----------------------------------------
usage as:
BXMr> find 叒
~ base word FIND code is exist?

all BXMr command, call ?|h|help
    "#;

#[allow(dead_code)]
pub const H_UPD: &str = r#"
-----------------------------------------
usage as:
BXMr> upd aaa 叒
~ base code&word UPGRADE the define in BXM codes

all BXMr command, call ?|h|help
    "#;

#[allow(dead_code)]
pub const H_DELE: &str = r#"
-----------------------------------------
usage as:
BXMr> dele aaa 叒
~ base code&word DELET the define from BXM codes

all BXMr command, call ?|h|help
    "#;

#[allow(dead_code)]
pub const H_AHEAD: &str = r#"
-----------------------------------------
usage as:
BXMr> ahead aaa 叒
~ base code&word AHEAD the define into top suggest list with rIME::BXM

all BXMr command, call ?|h|help
    "#;


pub fn toml2btmap(tfile:String) -> Option<BTreeMap<String, Vec<String>>> {
    println!("openning {}...",tfile);

    let mut file = File::open(tfile).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data: Value = toml::from_str(&contents).unwrap();

    let line_count = contents.lines().count();
    println!("reading : {} lines", line_count);
    let pb = ProgressBar::new(line_count as u64);

    let mut map = BTreeMap::new();
    //println!("load BXM code table {}...",tfile);
    
    for (key, value) in data.as_table().unwrap() {
        pb.inc(1);
        if let Some(array) = value.as_array() {
            //let vec = array.iter()
            //    .filter_map(|v| v.as_str())
            //    .map(String::from)
            //    .collect::<Vec<String>>();
            let mut vec = Vec::new();
            for v in array {
                if let Some(s) = v.as_str() {
                    vec.push(String::from(s));
                }
            }
            map.insert(key.clone(), vec.clone());
            //println!("insert: {}->{:#?}", key.clone(), vec.clone());
        }
    }

    //println!("{:#?}", map);

    Some(map)
}


fn load_toml(tfile:String) {
    //let file_path = tfile;
    let file = File::open(tfile).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(|line| {
        let line = line.ok()?;
        let trimmed = line.trim();
// clean empty and annotate...
        if trimmed.is_empty() || trimmed.starts_with("#") {
            None
        } else {
            Some(String::from(trimmed))
        }
    });
    let line_count = lines.count();
    println!("Line count: {}", line_count);
}


pub fn gen_bxm_all_codes(length: usize, 
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
        gen_bxm_all_codes(length - 1, key, gbxm);
    }
}

pub fn init2(codelen:usize) -> Option<BTreeMap<String, Vec<String>>> {
    let mut gbxm = BTreeMap::new();
    //util::generate_strings(4, String::new(), &mut gbxm);
    //generate_strings(4, String::new(), &mut gbxm);
    gen_bxm_all_codes(codelen, String::new(), &mut gbxm);
    println!("\t re-gen. all BXM code=> {} items", gbxm.clone().len());

    Some(gbxm)
}


pub fn yaload(yfile:String) -> Vec<(String, String)> {
    let file = fs::File::open(yfile).expect("Failed to open file");
    let reader = BufReader::new(file);
    //let path = Path::new(&yfile);
    //let file = File::open(path)?;
    //let reader = BufReader::new(file);
    //let lines = reader.lines().skip(9);
    let lines: Vec<_> = reader.lines().skip(9).collect(); // 转换成 Vec 对象
    let pb = ProgressBar::new(lines.len() as u64);
    //let pb = ProgressBar::new(lines.count() as u64);
    let mut entries = Vec::new();

    for line in lines {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            //let (code,word) = (parts[1].to_string(), parts[0].to_string());
            entries.push((parts[1].to_string(), parts[0].to_string()));
            //println!("load:\t{}:{}",parts[1].to_string(), parts[0].to_string());
        }
        pb.inc(1);
    }
    //println!("\t re-load rIME uasage .yaml data:{} lines\n",entries.len());

    entries
}

pub fn upd(key: &str, value: &str, gbxm: &mut BTreeMap<String, Vec<String>>) {
    match gbxm.get_mut(key) {
        Some(v) => {
            match v.iter().position(|x| x == value) {
                Some(_/* _pos */) => {
                    // Do nothing
                    {}
                },
                None => {
                    //v.push(value.to_owned()); // append the tail
                    // 插入到最前,但是, 导出 yaml 时反而在下层行
                    // 现在和 .yaml 前后一致
                    v.insert(0, value.to_owned()); 
                }
            }
        },
        None => {
            gbxm.insert(key.to_owned(), vec![value.to_owned()]);
        }
    }
}


pub fn put2(key: &str, value: &str, gbxm: &mut BTreeMap<String, Vec<String>>) {
    match gbxm.get_mut(key) {
        Some(v) => {
            match v.iter().position(|x| x == value) {
                Some(_/* _pos */) => {
                    // Do nothing
                    {}
                    //log::info!("{} already exists in {:?}", value, key);
                    //dbg!(format!("{} already exists in {:?}", value, key));
                },
                None => {
                    // 从 yaml 中加载时,应该和源数据顺序保持一致
                    v.push(value.to_owned()); // append the tail
                }
            }
        },
        None => {
            gbxm.insert(key.to_owned(), vec![value.to_owned()]);
        }
    }
}


pub fn del_item4list(list: &mut Vec<String>, word: &str) {
    list.retain(|x| x != word);
}

//use std::collections::BTreeMap;
pub fn replace_value(map: &mut BTreeMap<String, Vec<String>>, key: &str, new_value: Vec<String>) {
    map.entry(key.to_string())
        .and_modify(|value| *value = new_value.clone())
        .or_insert(new_value);
}

pub fn save2toml(code4btmap:BTreeMap<String, Vec<String>>, toml:String){
    // Convert BTreeMap to toml Value
    let toml_value = Value::try_from(code4btmap).unwrap();
    // Write toml Value to file
    let mut file = File::create(toml.clone()).unwrap();
    file.write_all(toml::to_string(&toml_value).unwrap().as_bytes()).unwrap();
    println!("\n\t saved -> {}",toml.clone());
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
        let _ = writeln!(handle, 
            "{} -> {:?}", key, values
        );
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
            for l in reader.lines().map_while(Result::ok) {
                if l.starts_with(&format!("{}=", key)) {
                    found_key = true;
                    new_lines.push_str(&format!("{}={} ", key, val));
                    println!("\n\t Updated .env item: {}={} ", key, val);
                } else {
                    new_lines.push_str(&l);
                    new_lines.push('\n');
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
    dotenv::from_path(f2denv).ok();
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
        Ok(_f2denv) => {
            let f2denv = ok_denv().unwrap().to_str().unwrap();
            println!("load .env <-{} ", f2denv);
            dotenv::from_path(f2denv).ok();
            //let val = std::env::var(key);

            match std::env::var(key) {
                Ok(val) => {
                    //println!("\n\t got: {}={}", key, val);
                    EnvResult::Success(key.to_owned(), val)
                },
                Err(_) => {
                    //println!("{} is not set in .env file", key);
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
                    let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

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
