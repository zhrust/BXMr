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

//use toml::Value;
use indicatif::ProgressBar;
//use indicatif::ProgressStyle;

use crate::inv::util;


pub fn flusht(ycodes:Vec<(String, String)>) {
    println!("reflush .toml with .yaml as {} codes", ycodes.len());

    //let ycodes = vec![("a".to_string(), "1".to_string()); 50]; // 假设有 50 个待处理项
    let pb = ProgressBar::new(ycodes.len() as u64);

    //let n = data.len();
    //let start = if n >= 10 { n - 10 } else { 0 };

    match util::chk_denv(util::ENV_TOML) {
        util::EnvResult::Success(_ekey, _toml) => {
            //println!("Key is OK");
            //println!("env hold:{}={}",dkey,dval);
            //let code4btmap = util::toml2btmap(_toml);
            match util::toml2btmap(_toml.clone()) {
                Some(mut code4btmap) => {
                    //for (i, (k, v)) in ycodes[0..9].iter().enumerate() {
                    for (i, (k, v)) in ycodes.iter().enumerate() {
                        //println!("{}: {}={}", i, k, v);
                        util::upd(k, v, &mut code4btmap);
                        pb.set_position((i + 1) as u64);
                        //pb.inc(1);
                    }
            pb.finish_with_message("done");

            util::save2toml(code4btmap,_toml);

            //// Convert BTreeMap to toml Value
            //let toml_value = Value::try_from(code4btmap).unwrap();
            //// Write toml Value to file
            //let mut file = File::create(_toml).unwrap();
            //file.write_all(toml::to_string(&toml_value).unwrap().as_bytes()).unwrap();

                },
                None => println!("Failed to parse TOML file"),
            }

        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }

}

//pub fn load(yaml: String) {
pub fn load() {
    match util::chk_denv(util::ENV_YAML) {
        util::EnvResult::Success(_ekey, _yaml) => {
            //println!("Key is OK");
            //println!("env hold:{}={}",dkey,dval);
            let _ycodes = util::yaload(_yaml);
            flusht(_ycodes);
        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
    //let res = format!("{}/{}",p2rime,bxm4mac);
    //log::debug!("load()<- {}", res);
    //yaload(res);

}



