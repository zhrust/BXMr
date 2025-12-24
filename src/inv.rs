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


/// Executes commands and returns true if data was modified (for auto-save)
pub fn fix(words:Vec<String>, bt4bxm:&mut BTreeMap<String, Vec<String>>) -> bool {
    let mut modified = false;
    
    match words.len() {
        1 => {
            match words[0].as_str() {
                "?" | "h" | "help" => {
                    println!("{}", _util::H_HELP);
                },
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
                _ => {
                    // 其它情况
                    println!("Unknown command: {}", words[0]);
                    println!("{}", _util::H_HELP);
                }
            }
        },
        2 => {
            match words[0].as_str() {
                "seek" => {
                    //seek::echo(words[1].clone());
                    seek::echo2(words[1].clone(),bt4bxm);
                    println!("{}", _util::H_MORE);
                },
                "find" => {
                    find::echo2(words[1].clone(),bt4bxm);
                    println!("{}", _util::H_MORE);
                },
                _ => {
                    // 其它情况
                    println!("Unknown command: {}", words[0]);
                    println!("{}", _util::H_HELP);
                }
            }
        },
        3 => {
            match words[0].as_str() {
                "cfg" => {
                    println!("Command: cfg");
                    cfg::set(words[1].clone(), words[2].clone());
                },
                "upd" => {
                    match upd::upd2(words[1].clone(), words[2].clone(), bt4bxm) {
                        Ok(changed) => {
                            if changed { modified = true; }
                        },
                        Err(e) => println!("Error: {}", e),
                    }
                    println!("{}", _util::H_MORE);
                },
                "dele" => {
                    match dele::kill2(words[1].clone(), words[2].clone(), bt4bxm) {
                        Ok(changed) => {
                            if changed { modified = true; }
                        },
                        Err(e) => println!("Error: {}", e),
                    }
                    println!("{}", _util::H_MORE);
                },
                "ahead" => {
                    match ahead::up1st2(words[1].clone(), words[2].clone(), bt4bxm) {
                        Ok(changed) => {
                            if changed { modified = true; }
                        },
                        Err(e) => println!("Error: {}", e),
                    }
                    println!("{}", _util::H_MORE);
                },
                _ => {
                    // 其它情况
                    println!("Unknown command: {}", words[0]);
                    println!("{}", _util::H_HELP);
                }
            }
        },
        _ => {
            // 其它情况
            println!("Unknown command");
            println!("{}", _util::H_HELP);
        }
    }//match cmds.len()
    
    modified
}

