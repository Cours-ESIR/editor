use std::{env, fs, io, path::PathBuf};

use chrono::{Datelike, Timelike};
use typst::foundations::Datetime;

pub fn now() -> Option<Datetime> {
    let now = chrono::Local::now().naive_utc();
    Datetime::from_ymd_hms(
        now.year(),
        now.month().try_into().ok()?,
        now.day().try_into().ok()?,
        now.hour().try_into().ok()?,
        now.minute().try_into().ok()?,
        now.second().try_into().ok()?,
    )
}

pub fn create_temp_folder() -> io::Result<PathBuf> {
    let temp_dir = env::temp_dir();
    let out_dir = temp_dir.join("typst-editor/"); // Temp folder
    if !out_dir.exists() {
        let _ = fs::create_dir_all(&out_dir)?;
    }
    Ok(out_dir)
}
