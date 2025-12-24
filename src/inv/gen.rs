use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::io::BufWriter;

use crate::inv::_util as util;

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

            writeln!(writer, "{}", util::RIME_HEAD).expect("Failed to write to file");
            for (value, key) in yavec {
                writeln!(writer, "{}\t{}", value, key).expect("Failed to write to file");
            }
            writer.flush().expect("Failed to flush buffer");

        },
        util::EnvResult::Failure(e) => {
            println!("failed: {}", e);
        },
    }
}




