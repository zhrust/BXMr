//use std::ffi::OsStr;
//use crate::inv::OsString;
//use crate::git::OsStr;

//use std::collections::HashMap;
//use std::collections::BTreeMap;
//use std::collections::HashSet;

use toml::Value;
use std::fs::File;
use std::io::Write;
//use std::collections::HashMap;
use crate::inv::util;


pub fn init(toml: String) {
    println!("init ~> {}", toml);
    log::debug!("init to:\n\t {}", toml);
    let mut gbxm = util::init2(2).unwrap();

    util::upd("zz", "双", &mut gbxm);
    util::upd("zz", "奻", &mut gbxm);
    util::upd("zz", "奻", &mut gbxm);
    let zz = gbxm.get("zz").unwrap();
    println!("zz -> {:?}", zz);

    //print_gbxm_sorted(&gbxm);

    // Convert BTreeMap to toml Value
    let toml_value = Value::try_from(gbxm).unwrap();
    // Write toml Value to file
    let mut file = File::create(toml).unwrap();
    file.write_all(toml::to_string(&toml_value).unwrap().as_bytes()).unwrap();

}




/*
fn print_gbxm_sorted(gbxm: &[(String, Vec<()>)]) {
    let mut gbxm_vec: Vec<_> = gbxm.iter().collect();
    gbxm_vec.sort_by_key(|(key, _)| key.to_owned());

    for (key, value) in gbxm_vec {
        println!("{}: {:?}", key, value);
    }
}
fn init2() -> HashMap<Vec<(String, Vec<(String)>)>> {
    let mut gbxm = HashMap::new();

    fn generate_strings(length: usize, prefix: String, gbxm: &mut Vec<(String, Vec<()>)>) {
        if length == 0 {
            return;
        }
        for c in BXMC.chars() {
            let key = prefix.clone() + &c.to_string();
            println!("{}", key);
            gbxm.push((key.clone(), Vec::new()));
            generate_strings(length - 1, key, gbxm);
        }
    }

    generate_strings(2, String::new(), &mut gbxm);
    println!("gen. all BXM code as {}", gbxm.len());

    upd("zz", "双", &mut gbxm);

    print_gbxm_sorted(&gbxm);

    Some(gbxm)
}
fn upd(key: &str, value: &str, gbxm: &mut HashMap<String, Vec<String>>) {
    if let Some(v) = gbxm.get_mut(key) {
        if v.contains(&value.to_owned()) {
            println!("{} already exists in {:?}", value, key);
        } else {
            v.push(value.to_owned());
            println!("Updated {} in {:?}", value, key);
        }
    } else {
        gbxm.insert(key.to_owned(), vec![value.to_owned()]);
        println!("Added {} to {:?}", value, key);
    }
}
 */