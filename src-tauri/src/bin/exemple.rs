use std::{env, fs, io, path::PathBuf};
use chrono::{Datelike, Timelike};
use clap::Parser;
use typst::{eval::Tracer, foundations::Datetime, visualize::Color};
use log::info;

#[path = "../fonts.rs"]
mod fonts;
#[path = "../packages.rs"]
mod packages;
#[path = "../world.rs"]
mod world;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the file to compile (.typ)
    file: PathBuf,

    /// Debug mode [info,debug]
    #[arg(short, long, default_value_t = log::LevelFilter::Debug)]
    debug: log::LevelFilter,
}

fn main() {
    let cli = Cli::parse();
    env_logger::builder().filter_level(cli.debug).init();

    let file = &cli.file;
    let filename = file.file_stem().unwrap().to_str().unwrap();
    let temp_folder = create_temp_folder().expect("Couln't create temp folder");

    info!("Creating typst World...");
    let world = world::EditorWorld::new(file);
    let mut tracer = Tracer::new();
    info!("Compiling typst Project...");
    let document = typst::compile(&world, &mut tracer).unwrap();
    info!("Project Compiled...");

    // Output PDF
    {
        let ident = world.input();
        let out_path = temp_folder.join(filename).with_extension("pdf");
        let buffer = typst_pdf::pdf(&document, ident.as_os_str().to_str(), now());
        fs::write(&out_path, buffer).unwrap();
        info!("Render PDF in {:?}", out_path);
    }

    // Output PNG
    {
        document.pages.iter().enumerate().for_each(|(i, page)| {
            let out_path = temp_folder
                .join(format!("{}_n{}", filename, i))
                .with_extension("png");
            let pixmap = typst_render::render(&page.frame, 144.0 / 72.0, Color::WHITE);
            pixmap.save_png(&out_path).unwrap();
            info!("Render PNG n°{} in {:?}", i, &out_path);
        })
    }

    // Output SVG
    {
        document.pages.iter().enumerate().for_each(|(i, page)| {
            let out_path = temp_folder
                .join(format!("{}_n{}", filename, i))
                .with_extension("svg");
            let buffer = typst_svg::svg(&page.frame);
            fs::write(&out_path, buffer).unwrap();
            info!("Render SVG n°{} in {:?}", i, &out_path);
        })
    }
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

fn create_temp_folder() -> io::Result<PathBuf> {
    let temp_dir = env::temp_dir();
    let out_dir = temp_dir.join("typst-editor/"); // Temp folder
    if !out_dir.exists() {
        let _ = fs::create_dir_all(&out_dir)?;
    }
    Ok(out_dir)
}
