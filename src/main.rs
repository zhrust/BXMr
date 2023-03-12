#![allow(unused)]
// main.rs
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

//use std::ffi::OsString;
//use clap::Parser;

mod inv;

#[tokio::main]
async fn main()  -> Result<()> {

    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    //#[cfg(feature = "with-file-history")]
    //if rl.load_history("history.txt").is_err() {
    //    println!("No previous history.");
    //}
    let mut hl: Vec<String> = Vec::new();
    loop {
        let readline = rl.readline("BXMr> ");
        match readline {
            Ok(line) => {
                //rl.add_history_entry(line.as_str())?;
                //println!("Line: {}", line.clone());
                hl.push(line.clone());


let mut cmds: Vec<String> = line
                .clone()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
//println!("{:?}", cmds);

inv::fix(cmds).await;

//inv::run().await;
/* #[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, clap::Parser)]
pub enum Commands {
    #[command(external_subcommand)]
    External(Vec<OsString>),
}
let args = Cli::parse();
log::debug!("src/inv/mod:{:?}", args);
 */

            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    //#[cfg(feature = "with-file-history")]
    //rl.save_history("history.txt");
    println!("BXMr this loop all commands:\n{:?}\n", hl);

    Ok(())
}
