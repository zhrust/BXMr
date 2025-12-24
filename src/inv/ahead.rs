//! Word priority adjustment module
//! 词条优先级调整模块

use std::collections::BTreeMap;

/// Moves word to front of code entry in BXM code table
/// Returns Ok(true) if data was modified, Ok(false) if code not found
pub fn up1st2(code: String
        , word: String
        , c4btmap: &mut BTreeMap<String, Vec<String>>
    ) -> anyhow::Result<bool> {

    if let Some(word5bxm) = c4btmap.get_mut(&code) {
        word5bxm.retain(|x| x != &word); // delete first
        println!("killed:\n\t{}->{:?}"
            , code
            , word5bxm
        );
        word5bxm.insert(0, word);   // insert at head
        println!("inserted: {} -> {:?}"
            , code
            , word5bxm
        );
        Ok(true)
    } else {
        anyhow::bail!("LOST code -> {}", code)
    }
}

