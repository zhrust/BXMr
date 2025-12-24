//! Initialization module for BXMr
//! BXMr 初始化模块

#![allow(dead_code)]

use toml::Value;
use std::fs::File;
use std::io::Write;
use crate::inv::_util as util;

pub fn init() {


    match util::chk_denv(util::ENV_TOML) {
        util::EnvResult::Success(dkey, dval) => {
            //println!("Key is OK");
            println!("env hold:\n\t{}={}",dkey,dval);
            let gbxm = util::init2(util::MBCL).unwrap();
            // Convert BTreeMap to toml Value
            let toml_value = Value::try_from(gbxm).unwrap();
            // Write toml Value to file
            let mut file = File::create(dval).unwrap();
            file.write_all(toml::to_string(&toml_value).unwrap().as_bytes()).unwrap();
        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
}



/* 
    //util::chk_denv("MY_VAR");
    //util::upd_denv("MY_VAR", "path/233/loc");
    //util::chk_denv("MY_VAR");
    //util::rmitem_denv("MY_VAR");
    
    util::upd("zz", "双", &mut gbxm);
    util::upd("zz", "奻", &mut gbxm);
    util::upd("zz", "奻", &mut gbxm);
    let zz = gbxm.get("zz").unwrap();
    println!("zz -> {:?}", zz);
 */
    

