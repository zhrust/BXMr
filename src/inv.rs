// git.rs
//use std::ffi::OsStr;
use std::ffi::OsString;
//use std::path::PathBuf;

use clap::Parser;
//use clap::{AppSettings, Parser, Subcommand};

pub mod util;
pub mod cfg;
pub mod env;

//pub mod usage;
pub mod init;
pub mod echo;
pub mod renew;
pub mod gen;

pub mod seek;
pub mod find;
pub mod upd;
pub mod dele;

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

    #[command(about = "print all BXM define as code:words")]
    Echo,

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

    #[command(about = "aaa 叒 <~ code word")]
    #[command(arg_required_else_help = false)]
    Upd {
        #[arg(value_name = "CODE")]
        code: String,
        #[arg(value_name = "WORD")]
        word: String,
    },
    #[command(about = "aaa 叒 <~ code word")]
    #[command(arg_required_else_help = false)]
    Dele {
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

pub fn run() {
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
        Commands::Echo  => echo::all(),
        Commands::Gen   => gen::exp(),
    // code,word
        Commands::Upd {
            code, word }=> upd::upd(code, word),
        Commands::Dele { 
            code, word }=> dele::kill(code, word),
    // one arg
//        Commands::Renew { yaml } => renew::load(yaml),
        Commands::Seek { 
            code }      => seek::echo(code),
        Commands::Find { 
            word }      => find::echo(word),
    // others
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }
}
