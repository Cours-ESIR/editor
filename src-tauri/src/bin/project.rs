use std::fs;

use editor::projet::Projet;
use log::info;
use typst_pdf::PdfOptions;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let temp_folder = editor::create_temp_folder().expect("Couln't create temp folder");

    let projet = Projet::default();
    let document = typst::compile(&projet).output.unwrap();

    {
        let out_path = temp_folder.join("Ourah").with_extension("pdf");

        let options = PdfOptions::default();

        let buffer = typst_pdf::pdf(&document, &options).unwrap();
        fs::write(&out_path, buffer).unwrap();
        info!("Render PDF in {:?}", out_path);
    }
}
