use std::collections::BTreeMap;

use crate::inv::_util as util;

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


pub fn kill2(code: String
        , word: String
        , c4btmap: &mut BTreeMap<String, Vec<String>>
    )-> Option<Box<BTreeMap<String, Vec<String>>>> {

    if let Some(word5bxm) = c4btmap.clone().get_mut(&code) {
        // println!("hold: {}", word5bxm.len());
        if word5bxm.len() == 0{
            println!("empty words, can not DELET\n\t:{} -> {}"
                , code, word);
            None

        }else{
            println!("already:\n\t{}->{:?}", code, word5bxm.clone());
            let _droped = word5bxm.retain(|x| x != &word);

            util::replace_value(c4btmap
                , &code
                , word5bxm.to_vec()
            );
            println!("killed:\n\t{}->{:?}"
                , code
                , c4btmap.get_mut(&code)
            );
            //util::upd(&code, &word, c4btmap);
            Some(Box::new(c4btmap.clone()))
        }

    }else{
        None
    }
}





