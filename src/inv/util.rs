#![allow(unused)]
use std::collections::BTreeMap;
use std::io::{self, Write};

pub const BXMC: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn generate_strings(length: usize, 
            prefix: String, 
            gbxm: &mut BTreeMap<String, Vec<String>>
        ) {
        if length == 0 {
            return;
        }
        for c in BXMC.chars() {
            let key = prefix.clone() + &c.to_string();
            //println!("{}", key);
            gbxm.insert(key.clone(), Vec::new());
            generate_strings(length - 1, key, gbxm);
        }
    }

pub fn init2(codelen:usize) -> Option<BTreeMap<String, Vec<String>>> {
    let mut gbxm = BTreeMap::new();
    //util::generate_strings(4, String::new(), &mut gbxm);
    //generate_strings(4, String::new(), &mut gbxm);
    generate_strings(codelen, String::new(), &mut gbxm);
    println!("gen. all BXM code as {}", gbxm.len());

    Some(gbxm)
}

pub fn upd(key: &str, value: &str, gbxm: &mut BTreeMap<String, Vec<String>>) {
    if let Some(v) = gbxm.get_mut(key) {
        if v.contains(&value.to_owned()) {
            println!("{} already exists in {:?}", value, key);
        } else {
            //v.push(value.to_owned());
            v.insert(0, value.to_owned());
            println!("Updated {} in {:?}", value, key);
        }
    } else {
        gbxm.insert(key.to_owned(), vec![value.to_owned()]);
        println!("Added {} to {:?}", value, key);
    }
}

pub fn print_gbxm_sorted(gbxm: &BTreeMap<String, Vec<String>>) {
    let mut sorted_keys: Vec<&String> = gbxm.keys().collect();
    sorted_keys.sort();

    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it

    for key in sorted_keys {
        let values = gbxm.get(key).unwrap();
        //println!("{} -> {:?}", key, values);
        writeln!(handle, 
            "{} -> {:?}", key, values
        ); // add `?` if you care about errors here
    }


}