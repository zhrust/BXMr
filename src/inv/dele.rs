use std::collections::BTreeMap;

use crate::inv::_util as util;

/*
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
                        println!("already:\n\t{}->}:?}", code, word5bxm.clone());
                        let _droped = word5bxm.retain(|x| x != &word);
                        util::replace_value(&mut c4btmap
                            , &code
                            , word5bxm.to_vec()
                        );
                        println!("killed:\n\t{}->{:?}"
                            , code
                            , c4btmap.get_mut(&code)
                        );
                        util::save2toml(c4btmap,_toml);
                    }
                } else {
                    println!("\n\t LOST code -> {} ", code);
                }
            },
            None => println!("Failed to parse TOML file"),
            }

        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
}
*/


/// Deletes word from code entry in BXM code table
/// Returns Ok(true) if data was modified, Ok(false) if no change or empty
pub fn kill2(code: String
        , word: String
        , c4btmap: &mut BTreeMap<String, Vec<String>>
    ) -> anyhow::Result<bool> {

    if let Some(word5bxm) = c4btmap.get_mut(&code) {
        if word5bxm.is_empty() {
            println!("empty words, can not DELET\n\t:{} -> {}"
                , code, word);
            Ok(false)
        } else {
            println!("already:\n\t{}->{:?}", code, word5bxm.clone());
            let original_len = word5bxm.len();
            word5bxm.retain(|x| x != &word);
            
            if word5bxm.len() < original_len {
                println!("killed:\n\t{}->{:?}"
                    , code
                    , word5bxm
                );
                Ok(true)
            } else {
                println!("word '{}' not found in code '{}'", word, code);
                Ok(false)
            }
        }
    } else {
        anyhow::bail!("LOST code -> {}", code)
    }
}





