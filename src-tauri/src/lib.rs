mod fonts;
mod packages;
pub mod projet;
pub mod world;

use chrono::{Datelike, Timelike};
use std::{env, fmt, fs, io, path::PathBuf};
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
        fs::create_dir_all(&out_dir)?;
    }
    Ok(out_dir)
}


pub enum EditorError {
    CompileError,
    CacheError,
    RenderUnknownPageError,
}

impl fmt::Display for EditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorError::CompileError => write!(f, "Compile Error"),
            EditorError::CacheError => write!(f, "Cache Error"),
            EditorError::RenderUnknownPageError => write!(f, "Page inconnu"),
        }
    }
}

impl serde::Serialize for EditorError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
