use crate::inv::util;

/* 
pub fn kill(code: String, word: String) {
    println!("src/inv/dele: {}", env!("CARGO_PKG_VERSION"));
    log::debug!("src/inv/dele: kill()\n\t {} \n\t{}", code, word);
    todo!("TODO: implement function");
}
use std::collections::BTreeMap;

fn replace_value(map: &mut BTreeMap<String, Vec<String>>, key: &str, new_value: Vec<String>) {
    map.entry(key.to_string())
        .and_modify(|value| *value = new_value.clone())
        .or_insert(new_value);
}
*/

pub async fn kill(code: String, word: String) {
// check .env is OK?
    match util::chk_denv(util::ENV_TOML) {
        util::EnvResult::Success(_ekey, _toml) => {
// Some() .toml can load Ok
        //match util::toml2btmap(_toml.clone()) {
        match util::async_toml2btmap(_toml.clone()).await {
            Some(mut c4btmap) => {
// Ok ? get_mut()
                if let Some(word5bxm) = c4btmap.clone().get_mut(&code) {
                    if word5bxm.len() == 0{
                        println!("empty words, can not DELET\n\t:{} -> {}"
                            , code, word);
                    }else{
                        println!("already:\n\t{}->{:?}", code, word5bxm.clone());
                        let _droped = word5bxm.retain(|x| x != &word);
                        //util::del_item4list(word5bxm.to_vec(), &word);
                        //println!("hold:{}",word5bxm.len());
                        util::replace_value(&mut c4btmap
                            , &code
                            , word5bxm.to_vec()
                        );
                        println!("killed:\n\t{}->{:?}", code, _droped.clone());
                    }
                } else {
                    println!("\n\t LOST code -> {} ", code);
                }
                util::save2toml(c4btmap,_toml);
            },
            None => println!("Failed to parse TOML file"),
            }

        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
}

