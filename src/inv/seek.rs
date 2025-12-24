use std::collections::BTreeMap;

pub fn echo2(code: String, bt4bxm:&mut BTreeMap<String, Vec<String>>) {

    if let Some(word5bxm) = bt4bxm.clone().get_mut(&code) {
        if word5bxm.is_empty(){
            println!("is new:{} -> {:?}", code, word5bxm.clone());
        }else{
            println!("{} already-> {:?}", code, word5bxm.clone());
            //println!("hold:{}",word5bxm.len());
        }
    } else {
        println!("\n\t LOST code -> {} ", code);
    }
    //util::save2toml(code4btmap,_toml);

}
