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


pub async fn fix(words:Vec<String>, bt4bxm:&mut BTreeMap<String, Vec<String>>) {
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

//#[command(name = "My CLI Tool"
//    , about = "A brief description of your tool"
//    , author = "Your Name")]
#[derive(Debug, Parser)]
#[command(author, version, about, 
    long_about = r#"BXMr Usage:
0: must setup .env for all Commands;
    $ bxmr cfg yaml path/2/u/local/bxm4zq2mac.dict.yaml
        ~ point u rIME-Squirrel usage .yaml
    $ bxmr cfg toml path/2/u/local/bxmr_loc_temp.toml
        ~ point u local temporary .toml, BXMr need this for cache data

> daily usage flow
1: seek the code is exist?
    $ bxmr seek aaa

2: if not exist, u can append it:
    $ bxmr upd aaa 叒

3: or find the word's code is exist? ~> find 字词
    or upd more code into temporary .toml

4: if enough now, must export to .yaml:
    $ bxmr gen

at last, always need usage rIME's re-deploy menu, 
    for load new code-table .yaml,
    so we can enjoy new BXM now ;-)
    "#)] // Read from `Cargo.toml`
pub struct Cli {
    //#[clap(flatten)]
    //verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Parser)]
pub enum Commands {
/* 
    #[command(about = "print all BXM define as code:words")]
    Echo,
 */
    #[command(about = "yaml|toml path/2/u/loc./AIM.yaml|toml ~ set rIME aim .yaml & BXMr usage .toml as ENV AT FIRST...")]
    #[command(arg_required_else_help = false)]
    Cfg {
        #[arg(value_name = "NAME")]
        name: String,
        #[arg(value_name = "PATH")]
        path: String,
    },

    #[command(about = "check bind ENV setting, work with coomad:cfg")]
    #[command(arg_required_else_help = false)]
    Env,

    #[command(about = "default -> ./log/bxm_dama_loc.toml, config by command: cfg")]
    #[command(arg_required_else_help = false)]
    Init ,

    #[command(about = "re-generating .yaml -> ~/Library/Rime/[U BXM].yaml, , config by command: cfg")]
    #[command(arg_required_else_help = false)]
    Gen,
/* 
    #[command(about = "print How to usage ~> tiny user manual...")]
    #[command(arg_required_else_help = false)]
    Usage, */

    #[command(about = "default -> ~/Library/Rime/[U BXM].yaml, , config by command: cfg")]
    #[command(arg_required_else_help = false)]
    Renew,

    #[command(about = "base code SEEK word is exist?")]
    #[command(arg_required_else_help = false)]
    Seek {
        #[arg(value_name = "CODE")]
        code: String,
    },

    #[command(about = "base word FIND code is exist?")]
    #[command(arg_required_else_help = false)]
    Find {
        #[arg(value_name = "WORD")]
        word: String,
    },

    #[command(about = "aaa 叒 <~ base code word UPGRADE the define in BXM")]
    #[command(arg_required_else_help = false)]
    Upd {
        #[arg(value_name = "CODE")]
        code: String,
        #[arg(value_name = "WORD")]
        word: String,
    },
    #[command(about = "aaa 叒 ~> base code word DELET the define from BXM")]
    #[command(arg_required_else_help = false)]
    Dele {
        #[arg(value_name = "CODE")]
        code: String,
        #[arg(value_name = "WORD")]
        word: String,
    },


    #[command(about = "aaa 叒 => base code word UP the word define 1st in BXM")]
    #[command(arg_required_else_help = false)]
    Ahead {
        #[arg(value_name = "CODE")]
        code: String,
        #[arg(value_name = "WORD")]
        word: String,
    },
/* 
    {
        #[arg(value_name = "YAML")]
        yaml: String,
    },
 */
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

pub async fn run() {
    let _guard = clia_tracing_config::build()
        .filter_level("debug") //fatal,error,warn,info,debug
        .with_ansi(true)
        .to_stdout(false)
        .directory("./log")
        .file_name("debug.log")
        .rolling("daily")
        .init();
    //log::debug!("src/inv/mod:{:?}", _guard);

    let args = Cli::parse();

    //log::debug!("src/inv/mod:{:?}", args);

    match args.command {
    // name.path
        Commands::Cfg {
            name, path }=> cfg::set(name, path),
    // not need arg.
        Commands::Env   => env::chk(),
        //Commands::Usage   => usage::echo(),

        Commands::Init => init::init(),
        Commands::Renew => renew::load(),
        //Commands::Echo  => echo::all(),
        Commands::Gen   => gen::exp().await,
    // code,word
        Commands::Upd {
            code, word }=> upd::upd(code, word).await,
        Commands::Ahead {
            code, word }=> ahead::up1st(code, word).await,
        Commands::Dele { 
            code, word }=> dele::kill(code, word).await,
    // one arg
//        Commands::Renew { yaml } => renew::load(yaml),
        Commands::Seek { 
            code }      => seek::echo(code).await,
        Commands::Find { 
            word }      => find::echo(word),
    // others
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }
}
