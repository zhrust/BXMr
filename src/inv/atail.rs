//! Word priority demotion module
//! 词条优先级降级模块

use std::collections::BTreeMap;

/// Moves word to end of code entry in BXM code table
/// 将词条移到编码列表末尾
/// Returns Ok(true) if data was modified, Err if code/word not found
pub fn down_last2(
    code: String,
    word: String,
    c4btmap: &mut BTreeMap<String, Vec<String>>,
) -> anyhow::Result<bool> {
    if let Some(word5bxm) = c4btmap.get_mut(&code) {
        // Check if word exists in the list
        if !word5bxm.contains(&word) {
            anyhow::bail!(
                "词条 '{}' 不存在于编码 '{}' 中\n  提示: 可使用 upd {} {} 新增",
                word, code, code, word
            )
        }
        
        word5bxm.retain(|x| x != &word);  // 先删除
        word5bxm.push(word.clone());      // 追加到末尾
        println!("moved to tail: {} -> {:?}", code, word5bxm);
        Ok(true)
    } else {
        anyhow::bail!("LOST code -> {}", code)
    }
}
