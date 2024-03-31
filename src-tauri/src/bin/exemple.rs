use clap::Parser;
use log::info;
use std::{env, fs, io, path::PathBuf};
use typst::{eval::Tracer, foundations::Datetime, visualize::Color};

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
    let temp_folder = editor::create_temp_folder().expect("Couln't create temp folder");

    info!("Creating typst World...");
    let world = editor::world::EditorWorld::new(file);
    let mut tracer = Tracer::new();
    info!("Compiling typst Project...");
    let document = typst::compile(&world, &mut tracer).unwrap();
    info!("Project Compiled...");

    // Output PDF
    {
        let ident = world.input();
        let out_path = temp_folder.join(filename).with_extension("pdf");
        let buffer = typst_pdf::pdf(
            &document,
            typst::foundations::Smart::Custom(ident.as_os_str().to_str().unwrap()),
            editor::now(),
        );
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


