use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;
//use toml::Value;

use crate::inv::_util as util;

//fn main() -> std::io::Result<()> {
pub async fn exp() {
    //println!("src/inv/gen: {}", env!("CARGO_PKG_VERSION"));
    //log::debug!("src/inv/gen: as {}", env!("CARGO_PKG_VERSION"));

    // check .env is OK?
    match util::chk_denv(util::ENV_TOML) {
        util::EnvResult::Success(_ekey, _toml) => {
    // Some() .toml can load Ok
        //match util::toml2btmap(_toml.clone()) {
        match util::async_toml2btmap(_toml.clone()).await {
            Some(c4btmap) => {
    // Ok ? get_mut()

    // chk & echo Yaml file path
            match std::env::var(util::ENV_YAML) {
                Ok(_yaml) => {
                    //println!("\n\t got: {}={}", util::ENV_YAML, val);
                    println!("export all BXM code-wors into:\n\t{}"
                            ,_yaml);
    // toml -> yaml exorpting...
    // Load data from toml file into a BTreeMap<String, Vec<String>>


        // 根据 toml 中的数据顺序转换
        let mut ordered_keys = vec![];
        for (key, _) in &c4btmap {
            ordered_keys.push(key.to_owned());
        }

        //ordered_keys.sort(); // 按字母顺序排序，可以更改排序规则

        // 将数据转换为 Vec<(String, String)>，其中每个元素表示一个键值对
        let mut vec: Vec<(String, String)> = vec![];
        for key in ordered_keys {
            if let Some(values) = c4btmap.get(&key) {
                if !values.is_empty() {
                    //for value in values {
                    // 从头部插入
                    for value in values.iter() {
                        vec.insert(0, (value.to_owned(), key.to_owned()));
                    }
                    //// 反向迭代收集
                    //for value in values.iter().rev() {　
                    //    vec.push((value.to_owned(), key.to_owned()));
                    //}
                }
            }
        }

        // 将数据写入到文件中
/* 
        let mut file = File::create(&_yaml);
        for (value, key) in vec {
            write!(file, "{}\t{}\n", value, key)?;
        }
 */
let mut file = File::create(&_yaml).expect("Failed to create file");

    write!(file, "{}\n", util::RIME_HEAD).expect("Failed to write to file");
for (value, key) in vec {
    write!(file, "{}\t{}\n", value, key).expect("Failed to write to file");
}

                },
                Err(_) => {
                    println!(r#"ALERT!
{} is not setup in .env
please usage cfg command to set at first, 
such as:
    $ bxmr cfg yaml path/2/rIME/usage/.yaml
                    "#, util::ENV_YAML);
                }
            }
                //util::save2toml(code4btmap,_toml);
            },
            None => println!("Failed to parse TOML file"),
            }

        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }



//    Ok(())
}



pub fn exp2(c4btmap: &mut BTreeMap<String, Vec<String>>){
    match util::chk_denv(util::ENV_YAML) {
        util::EnvResult::Success(_ekey, _yaml) => {
            println!("\n\t.env as: {} -> {}",_ekey,_yaml);

        // 将数据转换为 Vec<(String, String)>，其中每个元素表示一个键值对
        let mut yavec: Vec<(String, String)> = vec![];
        for (key, values) in c4btmap.iter() {
                if !values.is_empty() {
                    //println!("{:?}",values.iter());
                    //let mut reversed_values = values.clone();
                    //reversed_values.reverse();
                    //println!("{:?}",reversed_values);
                    //for value in reversed_values {
                    for value in values.iter() {
                        //println!("{:?}",value);
                        yavec.push((value.to_owned(), key.to_owned()));
                    }
                }
            }

            let file = File::create(&_yaml).expect("Failed to create file");
            let mut writer = BufWriter::new(file);

            write!(writer, "{}\n", util::RIME_HEAD).expect("Failed to write to file");
            for (value, key) in yavec {
                write!(writer, "{}\t{}\n", value, key).expect("Failed to write to file");
            }
            writer.flush().expect("Failed to flush buffer");

        },
        util::EnvResult::Failure(e) => {
            println!("failed: {}", e);
        },
    }
}




