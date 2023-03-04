//use std::ffi::OsStr;
//use crate::inv::OsString;
//use crate::git::OsStr;
use crate::inv::util;
pub fn set(name: String, path: String) {
    println!("set ENV:\n\t {}={}", name, path);

    if name =="yaml" {
        println!("upd..env => {}={}",util::ENV_YAML, &path);
        util::upd_denv(util::ENV_YAML, &path);
        
    }else if name =="toml" {
        println!("upd..env => {}={}",util::ENV_TOML, &path);
        util::upd_denv(util::ENV_TOML, &path);
        
    }else {
        println!(r#" ALERT! only support two option cfg:
        $ bxmr cfg [toml|yaml]
        means:
            + toml ~> point BXMr usage working file, store all BXM code-table
                for me is /opt/data/cfg/bxmr_dama_loc.toml
                
            + yaml ~> rIME usage system's Squirrel path, 
                in macOS default is ~//Library/Rime/
                for my customization file is bxm4zq2mac.dict.yaml 
        "#);
    }

    //log::debug!("src/inv/upd:\n\t {} \n\t{}", code, word);
}
