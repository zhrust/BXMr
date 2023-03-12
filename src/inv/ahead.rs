use crate::inv::_util as util;

pub async fn up1st(code: String, word: String) {
/* base poit code + word
update the code -> vec!words
- delet the word
- insert the word at the list tail
*/
    match util::chk_denv(util::ENV_TOML) {
        util::EnvResult::Success(_ekey, _toml) => {
// Some() .toml can load Ok
        match util::async_toml2btmap(_toml.clone()).await {
            Some(mut c4btmap) => {
// Ok ? get_mut()
                if let Some(word5bxm) = c4btmap.get_mut(&code) {
                    word5bxm.retain(|x| x != &word); // delet at first
                    println!("killed:\n\t{}->{:?}"
                        , code
                        , word5bxm
                    );
                    word5bxm.push(word);                   // append again
                    //util::upd(&code, &word, &mut c4btmap);
                    println!("inserted: {} -> {:?}"
                        , code
                        , c4btmap.get_mut(&code)
                        );
// save back .toml
                        util::save2toml(c4btmap,_toml);
                } 
            },
            None => println!("Failed to parse TOML file"),
            }
        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
}

