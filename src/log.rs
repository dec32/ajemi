use std::{env, fs, os::windows::fs::MetadataExt, path::PathBuf};
use chrono::Local;
use log::LevelFilter::*;

#[cfg(debug_assertions)]
const RELEASE: bool = false;
#[cfg(not(debug_assertions))]
const RELEASE: bool = true;

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
                "{} [{:<5}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                record.level(),
                message
            ))
        })
        .level(if RELEASE { Warn } else { Debug })
        .chain(fern::log_file(path)?)
        .apply()?;
    Ok(())
}

