use std::{env, fs, path::PathBuf};
use crate::{DEFAULT_CONF, EMOJI_SCHEMA, IME_NAME, SITELEN_SCHEMA};

pub fn install() -> Option<()>{
    let appdata = PathBuf::from(env::var("APPDATA").ok()?).join(IME_NAME);
    write_if_not_exist(appdata.join("conf.toml"), DEFAULT_CONF);
    write_if_not_exist(appdata.join("sitelen.schema"), SITELEN_SCHEMA);
    write_if_not_exist(appdata.join("emoji.schema"), EMOJI_SCHEMA);
    Some(())
}

fn write_if_not_exist(path: PathBuf, content: &str) {
    if path.exists() {
        return;
    }
    let _ = fs::write(path, content);
}