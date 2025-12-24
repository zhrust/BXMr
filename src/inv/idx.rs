//! Index management module for CSV persistence
//! 索引管理模块 (CSV 持久化)

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use crate::inv::_util;

/// Index map type: Char -> Code
/// 索引类型: 字符 -> 键码
pub type CharIndex = HashMap<char, String>;

/// Build index from BTreeMap and save to CSV
/// 从内存码表构建索引并保存为 CSV
/// CSV Format: char,code (No header)
pub fn build_and_save(bt: &BTreeMap<String, Vec<String>>) -> anyhow::Result<()> {
    // 1. Resolve CSV path
    let csv_path = resolve_idx_path()?;
    
    println!("Building index to {} ...", csv_path.display());

    // 2. Build index in memory (Char -> Code)
    // Strategy: If a char has multiple codes, keep the one with length 4 (MBCL) or the first one found.
    // 策略: 若一字多码，优先保留4码(MBCL)，否则保留第一个找到的。
    let mut index: CharIndex = HashMap::new();
    let mut count = 0;

    for (code, words) in bt {
        for word in words {
            // Only index single characters
            if word.chars().count() == 1 {
                let ch = word.chars().next().unwrap();
                
                // Rules for multiple codes:
                // 1. If not exists, insert.
                // 2. If exists, prefer length 4 (MBCL).
                // 3. If both are length 4 (or neither), keep existing (first encountered is usually higher priority in iteration? No, iteration order is by Key(Code)).
                // Since BTreeMap iterates codes sorted (aaa -> zzz), if 'aaa' and 'zzz' map to same char, 'aaa' comes first.
                // But we want the "standard" code. Usually 4-char code is standard in BXM.
                
                let should_insert = match index.get(&ch) {
                    None => true,
                    Some(existing_code) => {
                        // If existing is not 4 chars, and new one IS 4 chars, replace it.
                        existing_code.len() != _util::MBCL && code.len() == _util::MBCL
                    }
                };

                if should_insert {
                    index.insert(ch, code.clone());
                    count += 1;
                }
            }
        }
    }

    // 3. Save to CSV
    let mut file = File::create(&csv_path)?;
    for (ch, code) in index {
        writeln!(file, "{},{}", ch, code)?;
    }

    println!("Index built: {} items saved.", count);
    Ok(())
}

/// Load index from CSV
/// 从 CSV 加载索引
pub fn load() -> anyhow::Result<CharIndex> {
    let csv_path = resolve_idx_path()?;
    if !csv_path.exists() {
        anyhow::bail!("Index file not found: {}", csv_path.display());
    }

    let file = File::open(&csv_path)?;
    let reader = BufReader::new(file);
    let mut index = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((ch_str, code)) = line.split_once(',') {
            if let Some(ch) = ch_str.chars().next() {
                index.insert(ch, code.to_string());
            }
        }
    }

    Ok(index)
}

/// Helper: Resolve CSV path from ENV or default
fn resolve_idx_path() -> anyhow::Result<PathBuf> {
    let path_str = std::env::var(_util::ENV_IDX_CSV)
        .unwrap_or_else(|_| ".bxmr_idx.csv".to_string());
    Ok(PathBuf::from(path_str))
}
