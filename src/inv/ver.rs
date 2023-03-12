//use std::ffi::OsStr;
//use crate::inv::OsString;
//use crate::git::OsStr;

pub fn echo() {
    //println!("chk ENV:\n\t {}:{}", name, path);
    //println!("chk ENV...");
    println!("{: >16} {}","name:",       env!("CARGO_PKG_NAME"));
    println!("{: >16} {}","version:",    env!("CARGO_PKG_VERSION"));
    println!("{: >16} {}","desc:",       env!("CARGO_PKG_DESCRIPTION"));
    println!("{: >16} {}","authors:",    env!("CARGO_PKG_AUTHORS"));
    println!("{: >16} {}","homepage:",   env!("CARGO_PKG_HOMEPAGE"));
    println!("{: >16} {}","repo:",       env!("CARGO_PKG_REPOSITORY"));
    //log::debug!("src/inv/upd:\n\t {} \n\t{}", code, word);
}
/* 
env!("CARGO_PKG_VERSION") 
Cargo 也提供了其他一些与 Cargo.toml 文件对应的环境变量。这些环境变量以 CARGO_PKG_ 前缀开头，后跟相应的字段名。以下是一些常用的环境变量及其对应的字段：

CARGO_PKG_NAME: [package.name]，项目名称。
CARGO_PKG_DESCRIPTION: [package.description]，项目描述。
CARGO_PKG_AUTHORS: [package.authors]，项目作者列表，多个作者用逗号分隔。
CARGO_PKG_HOMEPAGE: [package.homepage]，项目主页 URL。
CARGO_PKG_REPOSITORY: [package.repository]，项目代码仓库 URL。
CARGO_PKG_LICENSE: [package.license]，项目许可证类型。
CARGO_PKG_VERSION: [package.version]，项目版本号。
CARGO_PKG_VERSION_MAJOR: [package.version]，项目版本号的主要版本号部分。
CARGO_PKG_VERSION_MINOR: [package.version]，项目版本号的次要版本号部分。
CARGO_PKG_VERSION_PATCH: [package.version]，项目版本号的修订版本号部分。
CARGO_PKG_VERSION_PRE: [package.version]，项目版本号的预发行版本号部分。
使用这些环境变量，您可以方便地在项目中获取 Cargo.toml 文件中的相关信息。
 */