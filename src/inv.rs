//! Command dispatcher module for BXMr
//! BXMr 命令分发模块

use std::collections::BTreeMap;

pub mod _util;
pub mod cfg;
pub mod env;
pub mod ver;

pub mod init;
pub mod renew;
pub mod gen;

pub mod seek;
pub mod find;
pub mod upd;
pub mod dele;
pub mod ahead;
pub mod atail;
pub mod idx;
pub mod apd;


/// Executes commands and returns true if data was modified (for auto-save)
/// 执行命令，返回是否修改了数据（用于自动保存）
pub fn fix(words: Vec<String>, bt4bxm: &mut BTreeMap<String, Vec<String>>) -> bool {
    let mut modified = false;
    
    if words.is_empty() {
        println!("{}", _util::H_HELP);
        return modified;
    }
    
    match words[0].as_str() {
        // 帮助命令 / Help commands
        "?" | "h" | "help" => {
            println!("{}", _util::H_HELP);
        },
        
        // 单参数命令 / Single-arg commands
        "env" => {
            env::chk();
            println!("{}", _util::H_MORE);
        },
        "ver" => {
            ver::echo();
            println!("{}", _util::H_MORE);
        },
        "renew" => {
            renew::load2btree();
            println!("{}", _util::H_MORE);
        },
        "gen" => {
            gen::exp2(bt4bxm);
            println!("{}", _util::H_MORE);
        },
        
        // cfg 命令: cfg <name> <path>
        "cfg" => {
            match words.len() {
                1 => {
                    println!("{}", _util::H_CFG);
                },
                2 => {
                    eprintln!("Error: cfg 命令需要两个参数");
                    eprintln!("  示例: cfg yaml /path/to/your.dict.yaml");
                    println!("{}", _util::H_CFG);
                },
                3 => {
                    cfg::set(words[1].clone(), words[2].clone());
                    println!("{}", _util::H_MORE);
                },
                _ => {
                    eprintln!("Error: cfg 命令参数过多");
                    eprintln!("  示例: cfg yaml /path/to/your.dict.yaml");
                    println!("{}", _util::H_CFG);
                }
            }
        },
        
        // seek 命令: seek <code>
        "seek" => {
            match words.len() {
                1 => {
                    eprintln!("Error: seek 命令需要编码参数");
                    eprintln!("  示例: seek aaa");
                    eprintln!("  功能: 查询编码对应的词条");
                },
                2 => {
                    seek::echo2(words[1].clone(), bt4bxm);
                    println!("{}", _util::H_MORE);
                },
                _ => {
                    eprintln!("Error: seek 命令只接受一个参数");
                    eprintln!("  示例: seek aaa");
                }
            }
        },
        
        // find 命令: find <word>
        "find" => {
            match words.len() {
                1 => {
                    eprintln!("Error: find 命令需要词条参数");
                    eprintln!("  示例: find 测试");
                    eprintln!("  功能: 查询词条对应的编码");
                },
                2 => {
                    find::echo2(words[1].clone(), bt4bxm);
                    println!("{}", _util::H_MORE);
                },
                _ => {
                    eprintln!("Error: find 命令只接受一个参数");
                    eprintln!("  示例: find 测试");
                }
            }
        },
        
        // upd 命令: upd <code> <word>
        "upd" => {
            match words.len() {
                1 => {
                    eprintln!("Error: upd 命令需要编码和词条参数");
                    eprintln!("  示例: upd aaa 叒");
                    eprintln!("  功能: 为编码添加新词条");
                },
                2 => {
                    eprintln!("Error: upd 命令缺少词条参数");
                    eprintln!("  示例: upd {} 词条", words[1]);
                },
                3 => {
                    match upd::upd2(words[1].clone(), words[2].clone(), bt4bxm) {
                        Ok(changed) => {
                            if changed { modified = true; }
                        },
                        Err(e) => eprintln!("Error: {}", e),
                    }
                    println!("{}", _util::H_MORE);
                },
                _ => {
                    eprintln!("Error: upd 命令参数过多");
                    eprintln!("  示例: upd aaa 叒");
                }
            }
        },
        
        // dele 命令: dele <code> <word>
        "dele" => {
            match words.len() {
                1 => {
                    eprintln!("Error: dele 命令需要编码和词条参数");
                    eprintln!("  示例: dele aaa 叒");
                    eprintln!("  功能: 从编码删除指定词条");
                },
                2 => {
                    eprintln!("Error: dele 命令缺少词条参数");
                    eprintln!("  示例: dele {} 词条", words[1]);
                },
                3 => {
                    match dele::kill2(words[1].clone(), words[2].clone(), bt4bxm) {
                        Ok(changed) => {
                            if changed { modified = true; }
                        },
                        Err(e) => eprintln!("Error: {}", e),
                    }
                    println!("{}", _util::H_MORE);
                },
                _ => {
                    eprintln!("Error: dele 命令参数过多");
                    eprintln!("  示例: dele aaa 叒");
                }
            }
        },
        
        // ahead 命令: ahead <code> <word>
        "ahead" => {
            match words.len() {
                1 => {
                    eprintln!("Error: ahead 命令需要编码和词条参数");
                    eprintln!("  示例: ahead aaa 叒");
                    eprintln!("  功能: 将词条移到编码首位");
                },
                2 => {
                    eprintln!("Error: ahead 命令缺少词条参数");
                    eprintln!("  示例: ahead {} 词条", words[1]);
                },
                3 => {
                    match ahead::up1st2(words[1].clone(), words[2].clone(), bt4bxm) {
                        Ok(changed) => {
                            if changed { modified = true; }
                        },
                        Err(e) => eprintln!("Error: {}", e),
                    }
                    println!("{}", _util::H_MORE);
                },
                _ => {
                    eprintln!("Error: ahead 命令参数过多");
                    eprintln!("  示例: ahead aaa 叒");
                }
            }
        },
        
        // atail 命令: atail <code> <word>
        "atail" => {
            match words.len() {
                1 => {
                    eprintln!("Error: atail 命令需要编码和词条参数");
                    eprintln!("  示例: atail aaa 叒");
                    eprintln!("  功能: 将词条移到编码列表末尾");
                },
                2 => {
                    eprintln!("Error: atail 命令缺少词条参数");
                    eprintln!("  示例: atail {} 词条", words[1]);
                },
                3 => {
                    match atail::down_last2(words[1].clone(), words[2].clone(), bt4bxm) {
                        Ok(changed) => {
                            if changed { modified = true; }
                        },
                        Err(e) => eprintln!("Error: {}", e),
                    }
                    println!("{}", _util::H_MORE);
                },
                _ => {
                    eprintln!("Error: atail 命令参数过多");
                    eprintln!("  示例: atail aaa 叒");
                }
            }
        },
        
        // idx 命令: 手动重建索引
        "idx" => {
            if let Err(e) = idx::build_and_save(bt4bxm) {
                eprintln!("Error building index: {}", e);
            }
        },

        // apd 命令: 交互式追加
        "apd" => {
            match apd::enter_loop(bt4bxm) {
                Ok(changed) => {
                    if changed { modified = true; }
                },
                Err(e) => eprintln!("Error in apd mode: {}", e),
            }
            println!("{}", _util::H_MORE);
        },
        
        // 未知命令 / Unknown command
        _ => {
            eprintln!("Unknown command: {}", words[0]);
            println!("{}", _util::H_HELP);
        }
    }
    
    modified
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a minimal BTreeMap for testing
    fn create_test_btmap() -> BTreeMap<String, Vec<String>> {
        let mut map = BTreeMap::new();
        map.insert("aaa".to_string(), vec!["测试".to_string(), "叒".to_string()]);
        map
    }

    #[test]
    fn test_empty_command_returns_false() {
        let mut bt = create_test_btmap();
        let result = fix(vec![], &mut bt);
        assert!(!result, "Empty command should return false");
    }

    #[test]
    fn test_help_commands_return_false() {
        let mut bt = create_test_btmap();
        assert!(!fix(vec!["?".to_string()], &mut bt));
        assert!(!fix(vec!["h".to_string()], &mut bt));
        assert!(!fix(vec!["help".to_string()], &mut bt));
    }

    #[test]
    fn test_cfg_without_args_returns_false() {
        let mut bt = create_test_btmap();
        let result = fix(vec!["cfg".to_string()], &mut bt);
        assert!(!result, "cfg without args should return false");
    }

    #[test]
    fn test_cfg_with_one_arg_returns_false() {
        let mut bt = create_test_btmap();
        let result = fix(vec!["cfg".to_string(), "yaml".to_string()], &mut bt);
        assert!(!result, "cfg with one arg should return false");
    }

    #[test]
    fn test_seek_without_args_returns_false() {
        let mut bt = create_test_btmap();
        let result = fix(vec!["seek".to_string()], &mut bt);
        assert!(!result, "seek without args should return false");
    }

    #[test]
    fn test_find_without_args_returns_false() {
        let mut bt = create_test_btmap();
        let result = fix(vec!["find".to_string()], &mut bt);
        assert!(!result, "find without args should return false");
    }

    #[test]
    fn test_upd_without_args_returns_false() {
        let mut bt = create_test_btmap();
        assert!(!fix(vec!["upd".to_string()], &mut bt));
        assert!(!fix(vec!["upd".to_string(), "aaa".to_string()], &mut bt));
    }

    #[test]
    fn test_dele_without_args_returns_false() {
        let mut bt = create_test_btmap();
        assert!(!fix(vec!["dele".to_string()], &mut bt));
        assert!(!fix(vec!["dele".to_string(), "aaa".to_string()], &mut bt));
    }

    #[test]
    fn test_ahead_without_args_returns_false() {
        let mut bt = create_test_btmap();
        assert!(!fix(vec!["ahead".to_string()], &mut bt));
        assert!(!fix(vec!["ahead".to_string(), "aaa".to_string()], &mut bt));
    }

    #[test]
    fn test_unknown_command_returns_false() {
        let mut bt = create_test_btmap();
        let result = fix(vec!["unknown_cmd".to_string()], &mut bt);
        assert!(!result, "Unknown command should return false");
    }

    #[test]
    fn test_atail_without_args_returns_false() {
        let mut bt = create_test_btmap();
        assert!(!fix(vec!["atail".to_string()], &mut bt));
        assert!(!fix(vec!["atail".to_string(), "aaa".to_string()], &mut bt));
    }

    #[test]
    fn test_atail_moves_word_to_end() {
        let mut bt = create_test_btmap();
        // bt has: "aaa" -> ["测试", "叒"]
        let result = fix(
            vec!["atail".to_string(), "aaa".to_string(), "测试".to_string()],
            &mut bt
        );
        assert!(result, "atail should return true when word moved");
        // After atail, "测试" should be at end: ["叒", "测试"]
        let words = bt.get("aaa").unwrap();
        assert_eq!(words.last().unwrap(), "测试");
    }
}
