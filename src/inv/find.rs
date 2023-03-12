use std::collections::BTreeMap;
//use toml::Value;
use indicatif::ProgressBar;

use crate::inv::_util as util;

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



pub fn echo2(word: String, bt4bxm:&mut BTreeMap<String, Vec<String>>) {
        for (code, words) in bt4bxm {
            if words.contains(&word) {
                println!("BXMr hold:{}<-{}",word,code);
                break;
            }
        }
}
