use std::{env, fs, os::windows::fs::MetadataExt, panic, path::PathBuf};
use chrono::Local;
use log::{Level, LevelFilter::*};

#[cfg(debug_assertions)]
const RELEASE: bool = false;
#[cfg(not(debug_assertions))]
const RELEASE: bool = true;

#[cfg(target_pointer_width = "64")]
const ARCHITECHTURE: &str = "x64";
#[cfg(target_pointer_width = "32")]
const ARCHITECHTURE: &str = "x86";

pub fn setup() {
    let _= _setup();
}

fn _setup() -> Result<(), fern::InitError>{
    let path = if let Ok(appdata) = env::var("APPDATA") {
        PathBuf::from(appdata).join("Ajemi")
    } else {
        return Ok(());
    };
    fs::create_dir_all(&path)?;
    let path = path.join("log.txt");
    if let Ok(meta) = fs::metadata(&path) {
        if meta.file_size() >= 5 * 1024 * 1024 {
            let _ = fs::remove_file(&path);
        }
    }
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{:<5}({})] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                record.level(),
                ARCHITECHTURE,
                message
            ))
        })
        .level(if RELEASE { Warn } else { Debug })
        .chain(fern::log_file(path)?)
        .apply()?;
    panic::set_hook(Box::new(|info|log::error!("Fatal error happend. {info}")));
    Ok(())
}

trait LevelExt {
    fn abbr(self) -> &'static str;
}

impl LevelExt for Level {
    fn abbr(self) -> &'static str {
        match self {
            Level::Error => "ERR",
            Level::Warn =>  "WRN",
            Level::Info =>  "INF",
            Level::Debug => "DBG",
            Level::Trace => "TRC",
        }
    }
}
