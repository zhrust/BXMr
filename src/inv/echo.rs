use std::collections::BTreeMap;

use crate::inv::util;

pub fn all() {
    let mut gbxm = BTreeMap::new();
    util::generate_strings(4, String::new(), &mut gbxm);
    println!("gen. all BXM code as {}", gbxm.len());
    util::print_gbxm_sorted(&gbxm);

}

