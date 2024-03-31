#![cfg_attr(all(not(debug_assertions)), windows_subsystem = "windows")]

use std::collections::HashMap;

use comemo::Prehashed;
use fonts::FontSearcher;
use log::info;
use projet::Projet;
use typst::{eval::Tracer, Library};
use typst_syntax::{FileId, VirtualPath};

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();
    tauri::Builder::default()
        // Register Command with Tauri App
        .invoke_handler(tauri::generate_handler![compile])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod fonts;
mod projet;
mod utils;

#[tauri::command]
fn compile(content: String) {
    // COmpile trés dirty pas beau vraiment.
    info!("compiling...");

    let mut tracer = Tracer::new();
    let temp_folder = utils::create_temp_folder().expect("Couln't create temp folder");

    let mut projet = Projet::new();
    projet.update_main(content);

    let document = typst::compile(&projet, &mut tracer).unwrap();

    {
        document.pages.iter().enumerate().for_each(|(i, page)| {
            let out_path = temp_folder
                .join(format!("{}_n{}", "main", i))
                .with_extension("png");
            let pixmap =
                typst_render::render(&page.frame, 144.0 / 72.0, typst::visualize::Color::WHITE);
            pixmap.save_png(&out_path).unwrap();

            // TODO return directly the pixmap or svg
            info!("Render PNG n°{} in {:?}", i, &out_path);
        })
    }
}
