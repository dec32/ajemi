use std::{env, fs, path::PathBuf};
use anyhow::Result;
use crate::{DEFAULT_CONF, IME_NAME};

pub fn install() -> Result<()>{
    let appdata = PathBuf::from(env::var("APPDATA")?).join(IME_NAME);
    if !appdata.exists() {
        fs::create_dir(&appdata)?;
    }
    let conf = appdata.join("conf.toml");
    if !conf.exists() {
        fs::write(conf, DEFAULT_CONF)?;
    }
    Ok(())
}