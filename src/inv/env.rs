use crate::inv::_util as util;

pub fn chk() {
    match util::chk_denv(util::ENV_YAML) {
        util::EnvResult::Success(_ekey, _toml) => {
            println!("{} {}",_ekey,_toml)
        },
        util::EnvResult::Failure(e) => println!("failed: {}", e),
    }
}

