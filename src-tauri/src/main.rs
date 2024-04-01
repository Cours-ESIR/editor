#![cfg_attr(all(not(debug_assertions)), windows_subsystem = "windows")]

use std::sync::RwLock;

use editor::{
    projet::{Projet, ProjetCache},
    EditorError,
};
use log::info;
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
fn update_project(content: String, state: tauri::State<AppState>) {
    info!("Update Project");
    let projet = &mut state.projet.write().unwrap();
    projet.update_main(content);
}

#[tauri::command]
fn compile_project(state: tauri::State<AppState>) -> Result<(), EditorError> {
    info!("Compile Project");
    let projet = state.projet.read().unwrap();
    let document =
        typst::compile(&*projet, &mut Tracer::new()).map_err(|_| EditorError::CompileError)?;
    let mut cache = state.cache.write().map_err(|_| EditorError::CacheError)?;
    cache.document = Some(document);
    Ok(())
}

#[tauri::command]
fn render_project(page: usize, state: tauri::State<AppState>) -> Result<String, EditorError> {
    info!("Render Project, page {}", page);
    let cache = state.cache.read().map_err(|_| EditorError::CacheError)?;
    let page = cache
        .document
        .as_ref()
        .and_then(|doc| doc.pages.get(page))
        .ok_or(EditorError::RenderUnknownPageError)?;
    let svg = typst_svg::svg(&page.frame);
    Ok(svg)
}
