// git.rs
//use std::ffi::OsStr;
use std::ffi::OsString;
//use std::path::PathBuf;

use clap::Parser;
//use clap::{AppSettings, Parser, Subcommand};

pub mod util;
pub mod echo;

pub mod init;
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
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
//#[clap(global_setting(AppSettings::PropagateVersion))]
//#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct Cli {
    //#[clap(flatten)]
    //verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Parser)]
pub enum Commands {

    #[command(about = "print all BXM code-words")]
    Echo,

    #[command(about = "gen. rIME .yaml...")]
    #[command(arg_required_else_help = false)]
    Gen,

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

    #[command(about = "path/2/[aim BXMr manag.].toml")]
    #[command(arg_required_else_help = false)]
    Init {
        #[arg(value_name = "TOML")]
        toml: String,
    },

    #[command(about = "base code SEEK word is there?")]
    #[command(arg_required_else_help = false)]
    Seek {
        #[arg(value_name = "CODE")]
        code: String,
    },

    #[command(about = "base word FIND code is there?")]
    #[command(arg_required_else_help = false)]
    Find {
        #[arg(value_name = "WORD")]
        word: String,
    },

    #[command(about = "path/2/[res BXM].yaml")]
    #[command(arg_required_else_help = false)]
    Renew,
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
    // not need arg.
        Commands::Gen   => gen::exp(),
        Commands::Echo  => echo::all(),
        Commands::Renew => renew::load(),
    // code,word
        Commands::Upd { 
            code, word }=> upd::upd(code, word),
        Commands::Dele { 
            code, word }=> dele::kill(code, word),
    // one arg
        Commands::Init { 
            toml }      => init::init(toml),
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
