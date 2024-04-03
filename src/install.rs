use std::{env, fs, path::PathBuf};
use crate::{DEFAULT_CONF, IME_NAME};

pub fn install() -> Option<()>{
    let appdata = PathBuf::from(env::var("APPDATA").ok()?).join(IME_NAME);
    let conf = appdata.join("conf.toml");
    if !conf.exists() {
        fs::write(conf, DEFAULT_CONF).ok()?;
    }
    Some(())
}