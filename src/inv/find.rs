//use std::fs::File;
//use std::io::Write;
//use toml::Value;
use indicatif::ProgressBar;

use crate::inv::util;

pub fn echo(word: String) {
    //println!("src/inv/seek: {}", env!("CARGO_PKG_VERSION"));
    // check .env is OK?
    match util::chk_denv(util::ENV_TOML) {
        util::EnvResult::Success(_ekey, _toml) => {
    // Some() .toml can load Ok
        match util::toml2btmap(_toml.clone()) {
            Some(c4btmap) => {
    // Ok ? get_mut()
                let pb = ProgressBar::new(c4btmap.len() as u64);
                println!("scanning : {} lines", c4btmap.len());
                for (code, words) in &c4btmap {
                    //println!("{}:", key);
                    //for value in values {
                    //    println!("  - {}", value);
                    //}
                    pb.inc(1);
                    if words.contains(&word) {
                        println!("BXMr hold:{}<-{}",word,code);
                        break;
                    } /* else {
                        //println!("{} 不存在于 {} 中",word,code);
                        //println!("{} already-> {:?}", code, words.clone());
                        dbg!(format!("BXMr {} already-> {:?}", 
                                code, words.clone()));
                    } */
                }
            },
            None => println!("Failed to parse TOML file"),
            }
        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
}

/*
let vec = vec!["啊啊啊", "奸", "叒"];
if vec.contains(&"叒") {
    println!("叒 存在于 Vec 中");
} else {
    println!("叒 不存在于 Vec 中");
}


*/