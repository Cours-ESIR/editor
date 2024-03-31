use std::fs;

use log::info;
use editor::projet::Projet;
use typst::eval::Tracer;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let temp_folder = editor::create_temp_folder().expect("Couln't create temp folder");

    let projet = Projet::default();
    let mut tracer = Tracer::new();
    let document = typst::compile(&projet, &mut tracer).unwrap();

    {
        let out_path = temp_folder.join("Ourah").with_extension("pdf");
        let buffer = typst_pdf::pdf(
            &document,
            typst::foundations::Smart::Auto,
            editor::now(),
        );
        fs::write(&out_path, buffer).unwrap();
        info!("Render PDF in {:?}", out_path);
    }
}


