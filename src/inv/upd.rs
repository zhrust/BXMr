use crate::inv::util;

/* 
pub fn upd(code: String, word: String) {
    println!("upd:\n\t {} \n\t{}", code, word);
    log::debug!("src/inv/upd:\n\t {} \n\t{}", code, word);
} */

pub async fn upd(code: String, word: String) {
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
                    //println!("hold:{}",word5bxm.len());
                    if word5bxm.contains(&word) {
                        println!("hold: {} -> {:?}\n\t no need upgrade code."
                            , code
                            , word5bxm.clone()
                            );
                        
                    }else{
                        util::upd(&code, &word, &mut c4btmap);
                        println!("inserted: {} -> {:?}"
                            , code
                            , c4btmap.get_mut(&code)
                            );
                        util::save2toml(c4btmap,_toml);

                    }
                } 
            },
            None => println!("Failed to parse TOML file"),
            }

        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
}

