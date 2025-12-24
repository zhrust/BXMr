use std::collections::BTreeMap;
use crate::inv::_util as util;

/// Updates BXM code table: adds word to code entry if not exists
/// Returns Ok(true) if data was modified, Ok(false) if no change needed
pub fn upd2(code: String
        , word: String
        , c4btmap: &mut BTreeMap<String, Vec<String>>
    ) -> anyhow::Result<bool> {

    if let Some(word5bxm) = c4btmap.get_mut(&code) {
        if word5bxm.iter().any(|w| w == &word) {
            println!(
                "hold: {} -> {:?}\n\t no need upgrade code.",
                code,
                word5bxm.clone()
            );
            Ok(false)
        } else {
            util::upd(&code, &word, c4btmap);
            println!(
                "inserted: {} -> {:?}",
                code,
                c4btmap.get(&code).unwrap().clone()
            );
            Ok(true)
        }
    } else {
        // Code not found - insert new entry
        util::upd(&code, &word, c4btmap);
        println!(
            "inserted new code: {} -> {:?}",
            code,
            c4btmap.get(&code).unwrap().clone()
        );
        Ok(true)
    }
}







