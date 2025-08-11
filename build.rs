use std::{env, io};

use winres::WindowsResource;

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return Ok(());
    }
    WindowsResource::new()
        .set_icon_with_id("res/tray_lite.ico", "0")
        .set_icon_with_id("res/tray_dark.ico", "1")
        .compile()
}
