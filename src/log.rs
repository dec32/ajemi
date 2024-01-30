use std::{env, fs, path::PathBuf};
use chrono::Local;
use log::LevelFilter::*;

#[cfg(debug_assertions)]
const RELEASE: bool = false;
#[cfg(not(debug_assertions))]
const RELEASE: bool = true;

pub fn setup() -> Result<(), fern::InitError>{
    let path = if let Ok(appdata) = env::var("APPDATA") {
        PathBuf::from(appdata).join("Ajemi")
    } else {
        return Ok(());
    };
    fs::create_dir_all(&path)?;
    let path = path.join("log.txt");
    
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{:<5}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                record.level(),
                message
            ))
        })
        .level(if RELEASE { Warn } else { Trace })
        .chain(fern::log_file(path)?)
        .apply()?;
    Ok(())
}

