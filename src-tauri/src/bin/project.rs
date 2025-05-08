use std::fs;

use editor::projet::Projet;
use log::info;
use typst::eval::Tracer;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let temp_folder = editor::create_temp_folder().expect("Couln't create temp folder");

    let projet = Projet::default();
    let mut tracer = Tracer::new();
    let document = typst::compile(&projet).unwrap();

    {
        let out_path = temp_folder.join("Ourah").with_extension("pdf");
        let buffer = typst_pdf::pdf(&document, typst::foundations::Smart::Auto);
        fs::write(&out_path, buffer).unwrap();
        info!("Render PDF in {:?}", out_path);
    }
}
