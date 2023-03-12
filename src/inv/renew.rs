//#![allow(unused)]
//use std::error::Error;
//use std::fs;
//use std::fs::File;
//use std::io::{self, Write};
//use std::io::Write;
//use std::io::BufReader;
//use std::io::BufRead;
//use std::io::{BufRead, BufReader};
//use std::path::Path;
use std::thread;
use std::time::Duration;
use std::collections::BTreeMap;

//use toml::Value;
use indicatif::ProgressBar;
//use indicatif::ProgressStyle;

use crate::inv::_util as util;


pub fn flusht(ycodes:Vec<(String, String)>) {
    println!("reflush .toml with .yaml as {} codes", ycodes.len());

    //let ycodes = vec![("a".to_string(), "1".to_string()); 50]; // 假设有 50 个待处理项
    let pb = ProgressBar::new(ycodes.len() as u64);

    //let n = data.len();
    //let start = if n >= 10 { n - 10 } else { 0 };

    match util::chk_denv(util::ENV_TOML) {
        util::EnvResult::Success(_ekey, _toml) => {
            //let code4btmap = util::toml2btmap(_toml);
            match util::toml2btmap(_toml.clone()) {
                Some(mut code4btmap) => {
                    for (i, (k, v)) in ycodes.iter().enumerate() {
                        util::upd(k, v, &mut code4btmap);
                        //pb.set_position((i + 1) as u64);
                        pb.inc(1);
                    }
            pb.finish_with_message("load all Yaml data.");

            util::save2toml(code4btmap,_toml);

                },
                None => println!("Failed to parse TOML file"),
            }

        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }

}

pub fn load() {
    match util::chk_denv(util::ENV_YAML) {
        util::EnvResult::Success(_ekey, _yaml) => {
            //println!("env hold:{}={}",dkey,dval);
            let _ycodes = util::yaload(_yaml);
            flusht(_ycodes);
        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
}
/* 
//pub fn load2btree() -> Option<&'static mut BTreeMap<String, Vec<String>>> {
//pub fn load2btree() -> Option<Box<BTreeMap<String, Vec<String>>>> {
pub fn load2btree() -> Option<&'static mut BTreeMap<String, Vec<String>>> {
    match util::chk_denv(util::ENV_YAML) {
        util::EnvResult::Success(_ekey, _yaml) => {
            println!("env hold:{}={}",_ekey,_yaml);
            let _ycodes = util::yaload(_yaml);

            //let mut _bxm = util::init2(4);
            let mut _bxm = Box::new(util::init2(4));

            let mut c4bxm = flush4bxm(_ycodes, _bxm.as_mut().unwrap());

            Some(c4bxm)

        },
        util::EnvResult::Failure(e) => {
            println!("failed: {}", e);
            None
        },
    }
} */


pub fn load2btree() -> Option<Box<BTreeMap<String, Vec<String>>>> {
    match util::chk_denv(util::ENV_YAML) {
        util::EnvResult::Success(_ekey, _yaml) => {
            println!("env hold:{}={}",_ekey,_yaml);
            let _ycodes = util::yaload(_yaml);

            let mut _bxm = util::init2(4);

            let c4bxm = flush4bxm(_ycodes, _bxm.as_mut().unwrap());

            Some(Box::new(c4bxm.clone()))
        },
        util::EnvResult::Failure(e) => {
            println!("failed: {}", e);
            None
        },
    }
}


/*USAGE:
if let Some(btree) = load2btree() {
    let bt = *btree;
    // 在这里使用 bt，它是一个 BTreeMap<String, Vec<String>> 类型的对象
}

let btree = match load2btree() {
    Some(map) => map,
    None => {
        println!("Failed to load BTreeMap");
        // 使用默认值
        &mut BTreeMap::new()
    },
};

// 对 BTreeMap 进行修改
btree.insert("key".to_string(), vec!["value1".to_string(), "value2".to_string()]);

*/


pub fn flush4bxm(ycodes:Vec<(String, String)>,
            code4btmap: &mut BTreeMap<String, Vec<String>>
        )-> &mut BTreeMap<String, Vec<String>> {
    println!("fix memory BTreeMap obj. from .yaml as {} codes", ycodes.len());

    let pb = ProgressBar::new(ycodes.len() as u64);

    //let mut code4btmap;
    for (i, (k, v)) in ycodes.iter().enumerate() {
        //util::upd(k, v, code4btmap);
        //thread::sleep(Duration::from_millis(4));
        util::upd(k, v, code4btmap);
        pb.inc(1);
    }
    pb.finish_with_message("load all Yaml data.");

    code4btmap
}


