//! Interactive Append Command (apd)
//! 交互式追加指令 (Sub-shell)

use std::collections::BTreeMap;
use inquire::{Text, Confirm};
use crate::inv::idx;

/// Enter the apd interactive loop
/// 进入 apd 交互循环
pub fn enter_loop(bt: &mut BTreeMap<String, Vec<String>>) -> anyhow::Result<bool> {
    println!("Entering apd mode (Type 'exit' or Ctrl+C to quit)");
    
    // 1. Load or Build Index
    // 尝试加载索引，如果失败则现场构建
    let index = match idx::load() {
        Ok(idx) => {
            println!("Loaded index with {} chars.", idx.len());
            idx
        },
        Err(_) => {
            println!("Index not found, building temporary index...");
            idx::build_and_save(bt)?;
            idx::load()?
        }
    };

    let mut modified = false;

    // 2. Interactive Loop
    loop {
        // Prompt for input
        let input_res = Text::new("BXMr [apd]>")
            .with_help_message("输入词组 (如: 弓木奈於Nao)")
            .prompt();

        let input = match input_res {
            Ok(s) => s.trim().to_string(),
            Err(_) => break, // Ctrl+C or Error
        };

        if input.is_empty() { continue; }
        if input.eq_ignore_ascii_case("exit") { break; }

        // 3. Process Input
        match generate_code(&input, &index) {
            Ok((code, source_info)) => {
                println!("建议键码: {}", code);
                println!("组码来源:\n{}", source_info);

                // 4. Confirm
                let confirm = Confirm::new("是否同意?")
                    .with_default(true)
                    .with_help_message("y: 确认追加 / n: 放弃")
                    .prompt();

                match confirm {
                    Ok(true) => {
                        // Append to BTreeMap
                        bt.entry(code.clone())
                            .or_default()
                            .push(input.clone());
                        println!("追加成功: {} {}", code, input);
                        modified = true;
                    },
                    Ok(false) => println!("已放弃。"),
                    Err(_) => break,
                }
            },
            Err(e) => {
                eprintln!("无法生成键码: {}", e);
            }
        }
    }
    
    Ok(modified)
}

/// Generate code for the input word
/// 生成键码算法
fn generate_code(input: &str, index: &idx::CharIndex) -> anyhow::Result<(String, String)> {
    // Separate chars and suffix (e.g. "弓木奈於Nao" -> chars=['弓','木','奈','於'], suffix="Nao")
    // Strategy: Take all leading CJK chars? Or just take all non-ascii?
    // BXM usually applies to Chinese chars.
    // Simple approach: Take chars until an ASCII char is found?
    // Rules says: "如果词组末尾包含英文...要原样在最终键值中" (implies Value, not Code).
    // The Input `jwtj` for `自强不息yyds` implies the code `jwtj` is derived ONLY from `自强不息`.
    
    let chars: Vec<char> = input.chars()
        .filter(|c| !c.is_ascii()) // Filter out ASCII (english/numbers/punctuation)
        .collect();
    
    if chars.is_empty() {
        anyhow::bail!("输入未包含有效汉字");
    }

    let len = chars.len();
    let mut code_parts = Vec::new();
    let mut info_lines = Vec::new();

    // Helper to get code for char
    let get_code = |ch: char| -> anyhow::Result<String> {
        index.get(&ch).cloned().ok_or_else(|| anyhow::anyhow!("字 '{}' 无对应键码", ch))
    };

    match len {
        // 1 char: Full code
        1 => {
            let c = chars[0];
            let code = get_code(c)?;
            code_parts.push(code.clone());
            info_lines.push(format!("{} (全码) -> {}", c, code));
        },
        // 2 chars: 2 + 2
        2 => {
            for &c in chars.iter() {
                let full = get_code(c)?;
                let part = take_chars(&full, 2);
                code_parts.push(part.clone());
                info_lines.push(format!("{} {} (取前2) -> {}", full, c, part));
            }
        },
        // 3 chars: 2 + 1 + 1
        3 => {
            let rules = [2, 1, 1];
            for (i, &c) in chars.iter().enumerate() {
                let full = get_code(c)?;
                let part = take_chars(&full, rules[i]);
                code_parts.push(part.clone());
                info_lines.push(format!("{} {} (取前{}) -> {}", full, c, rules[i], part));
            }
        },
        // 4 chars: 1 + 1 + 1 + 1
        4 => {
            for c in chars {
                let full = get_code(c)?;
                let part = take_chars(&full, 1);
                code_parts.push(part.clone());
                info_lines.push(format!("{} {} (取前1) -> {}", full, c, part));
            }
        },
        // > 4 chars: 1 + 1 + 1 + ... + last(1)
        _ => {
            // Idx: 0, 1, 2, Last
            let indices = [0, 1, 2, len - 1];
            for &i in &indices {
                let c = chars[i];
                let full = get_code(c)?;
                let part = take_chars(&full, 1);
                code_parts.push(part.clone());
                info_lines.push(format!("{} {} (取前1) -> {}", full, c, part));
            }
        }
    }

    let final_code = code_parts.join("");
    Ok((final_code, info_lines.join("\n")))
}

fn take_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_mock_index() -> idx::CharIndex {
        let mut idx = HashMap::new();
        idx.insert('弓', "wzh".to_string());
        idx.insert('木', "mhi".to_string());
        idx.insert('奈', "rgg".to_string());
        idx.insert('於', "ihvg".to_string());
        idx.insert('自', "jwtj".to_string());
        idx.insert('强', "wkyy".to_string());
        idx.insert('不', "thuu".to_string());
        idx.insert('息', "thnn".to_string());
        idx.insert('才', "ftt".to_string());
        idx.insert('是', "jgh".to_string());
        idx.insert('真', "fhuu".to_string());
        idx
    }

    #[test]
    fn test_generate_code_single_char() {
        let idx = create_mock_index();
        let (code, _) = generate_code("弓", &idx).unwrap();
        assert_eq!(code, "wzh");
    }

    #[test]
    fn test_generate_code_2_chars() {
        let idx = create_mock_index();
        // 弓(wzh) 木(mhi) -> wz + mh -> wzmh (First 2 chars of each code)
        let (code, _) = generate_code("弓木", &idx).unwrap();
        assert_eq!(code, "wzmh");
    }

    #[test]
    fn test_generate_code_3_chars() {
        let idx = create_mock_index();
        // 弓(wzh) 木(mhi) 奈(rgg) -> wz + m + r -> wzmr
        let (code, _) = generate_code("弓木奈", &idx).unwrap();
        assert_eq!(code, "wzmr");
    }

    #[test]
    fn test_generate_code_4_chars() {
        let idx = create_mock_index();
        // 自强不息 -> j + w + t + t -> jwtt
        let (code, _) = generate_code("自强不息", &idx).unwrap();
        assert_eq!(code, "jwtt");
    }

    #[test]
    fn test_generate_code_more_than_4_chars() {
        let idx = create_mock_index();
        // 自强不息才是真 (7 chars) -> First(j) + Second(w) + Third(t) + Last(f) -> jwtf
        let (code, _) = generate_code("自强不息才是真", &idx).unwrap();
        assert_eq!(code, "jwtf");
    }

    #[test]
    fn test_generate_code_with_suffix() {
        let idx = create_mock_index();
        // 弓木奈於Nao -> 弓木奈於 (4 chars) -> w + m + r + i -> wmri
        let (code, _) = generate_code("弓木奈於Nao", &idx).unwrap();
        assert_eq!(code, "wmri");
    }

    #[test]
    fn test_take_chars() {
        assert_eq!(take_chars("abcdef", 2), "ab");
        assert_eq!(take_chars("abc", 5), "abc");
        assert_eq!(take_chars("a", 1), "a");
    }
}
