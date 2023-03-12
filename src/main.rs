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
    let mut hl: Vec<String> = Vec::new();


if let Some(btree) = inv::renew::load2btree() {
    let mut bt4bxm = *btree;
    // 在这里使用 bt，它是一个 BTreeMap<String, Vec<String>> 类型的对象
//}
    println!("{}", inv::_util::H_MORE);

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
            inv::fix(cmds, &mut bt4bxm).await;
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

}//inv::renew::load2btree()
    //#[cfg(feature = "with-file-history")]
    //rl.save_history("history.txt");
    println!("BXMr this loop all commands:\n{:?}\n", hl);

    Ok(())
}
