use std::{env, fs, io, path::PathBuf};

use chrono::{Datelike, Timelike};
use log::info;
use projet::Projet;
use typst::{eval::Tracer, foundations::Datetime};

#[path = "../fonts.rs"]
mod fonts;
#[path = "../packages.rs"]
mod packages;
#[path = "../projet.rs"]
mod projet;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let temp_folder = create_temp_folder().expect("Couln't create temp folder");

    let projet = Projet::new();
    let mut tracer = Tracer::new();
    let document = typst::compile(&projet, &mut tracer).unwrap();

    {
        let out_path = temp_folder.join("Ourah").with_extension("pdf");
        let buffer = typst_pdf::pdf(
            &document,
            typst::foundations::Smart::Auto,
            now(),
        );
        fs::write(&out_path, buffer).unwrap();
        info!("Render PDF in {:?}", out_path);
    }
}

fn create_temp_folder() -> io::Result<PathBuf> {
    let temp_dir = env::temp_dir();
    let out_dir = temp_dir.join("typst-editor/"); // Temp folder
    if !out_dir.exists() {
        let _ = fs::create_dir_all(&out_dir)?;
    }
    Ok(out_dir)
}

fn now() -> Option<Datetime> {
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
