#![allow(unused)]
//use std::ffi::OsStr;
use std::collections::BTreeMap;
use std::ffi::OsString;
//use std::path::PathBuf;

use clap::Parser;
//use clap::{AppSettings, Parser, Subcommand};

pub mod _util;
pub mod cfg;
pub mod env;
pub mod ver;

//pub mod usage;
pub mod init;
//pub mod echo;
pub mod renew;
pub mod gen;

pub mod seek;
pub mod find;
pub mod upd;
pub mod dele;
pub mod ahead;


pub fn fix(words:Vec<String>, bt4bxm:&mut BTreeMap<String, Vec<String>>) {
    //println!("{:?}", words);
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
                    //println!("Code value: {}", words[1]);
                    //println!("Text: {}", words[2]);
                    cfg::set(words[1].clone(), words[2].clone());
                },
                "upd" => {
                    //bt4bxm = upd::upd2(words[1].clone()
                    //    , words[2].clone()
                    //    , &mut bt4bxm);

if let Some(bt4bxm) = upd::upd2(words[1].clone()
                        , words[2].clone()
                        , bt4bxm) {
    let bt4bxm = *bt4bxm;
    // 在这里使用 bt，它是一个 BTreeMap<String, Vec<String>> 类型的对象
    }
                    println!("{}", _util::H_MORE);
                },
                "dele" => {
if let Some(bt4bxm) = dele::kill2(words[1].clone()
                        , words[2].clone()
                        , bt4bxm) {
    let bt4bxm = *bt4bxm;
    // 在这里使用 bt，它是一个 BTreeMap<String, Vec<String>> 类型的对象
    }

                    println!("{}", _util::H_MORE);

                },
                "ahead" => {

if let Some(bt4bxm) = ahead::up1st2(words[1].clone()
                        , words[2].clone()
                        , bt4bxm) {
    let bt4bxm = *bt4bxm;
    // 在这里使用 bt，它是一个 BTreeMap<String, Vec<String>> 类型的对象
    }

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

}

