//! Word deletion module
//! 词条删除模块

use std::collections::BTreeMap;

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





