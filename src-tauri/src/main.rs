#![cfg_attr(all(not(debug_assertions)), windows_subsystem = "windows")]

use std::sync::RwLock;

use editor::projet::{Projet, ProjetCache};
use log::{debug, info};
use typst::eval::Tracer;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    tauri::Builder::default()
        .manage(AppState::default())
        // Register Command with Tauri App
        .invoke_handler(tauri::generate_handler![
            update_project,
            compile_project,
            render_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Default)]
struct AppState {
    projet: RwLock<Projet>,
    cache: RwLock<ProjetCache>,
}

#[tauri::command]
fn update_project<'a>(content: String, state: tauri::State<'a, AppState>) {
    info!("Update Project");
    let projet = &mut state.projet.write().unwrap();
    projet.update_main(content); // TODO check error
}

#[tauri::command]
fn compile_project(state: tauri::State<AppState>) {
    info!("Compile Project");
    let projet = state.projet.read().unwrap();
    let document = typst::compile(&*projet, &mut Tracer::new()).unwrap(); // TODO check error
    state.cache.write().unwrap().document = Some(document); // TODO check error
}

#[tauri::command]
fn render_project(page: usize, state: tauri::State<AppState>) -> String {
    info!("Render Project, page {}", page);
    let cache = state.cache.read().unwrap(); // TODO check error
    dbg!(cache.document.as_ref().unwrap().pages.get(page));
    if let Some(page) = cache.document.as_ref().and_then(|doc| doc.pages.get(page)) {
        let svg = typst_svg::svg(&page.frame);
        dbg!(&svg);
        return svg;
    }
    debug!("Failed to Render, page {}", page);
    return "".to_string();
}
