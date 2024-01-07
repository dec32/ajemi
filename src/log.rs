use std::{fs::File, io::Write, time::SystemTime};

pub fn debug(text: &str) {
    log(&format!("[DEBUG] {text}"));
}

pub fn error(text: &str) {
    log(&format!("[ERROR] {text}"));
}

fn log(text: &str) {
    let mut file = File::options().write(true).append(true).open("C:\\ajemi.log").unwrap();
    file.write(text.as_bytes()).unwrap();
    file.write(b"\n").unwrap();
    file.flush().unwrap();
}