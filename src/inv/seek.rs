//use std::fs::File;
//use std::io::Write;
//use toml::Value;

use crate::inv::_util as util;

pub async fn echo(code: String) {
    //println!("src/inv/seek: {}", env!("CARGO_PKG_VERSION"));

    // check .env is OK?
    match util::chk_denv(util::ENV_TOML) {
        util::EnvResult::Success(_ekey, _toml) => {
    // Some() .toml can load Ok
        //match util::toml2btmap(_toml.clone()) {
        match util::async_toml2btmap(_toml.clone()).await {
            Some(mut c4btmap) => {
    // Ok ? get_mut()
                if let Some(word5bxm) = c4btmap.get_mut(&code) {
                    if word5bxm.len() == 0{
                        println!("is new:{} -> {:?}", code, word5bxm.clone());
                    }else{
                        println!("{} already-> {:?}", code, word5bxm.clone());
                        //println!("hold:{}",word5bxm.len());
                    }
                } else {
                    println!("\n\t LOST code -> {} ", code);
                }
                //util::save2toml(code4btmap,_toml);
            },
            None => println!("Failed to parse TOML file"),
            }

        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
}

