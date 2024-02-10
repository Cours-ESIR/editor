#![cfg_attr(all(not(debug_assertions)), windows_subsystem = "windows")]

// Our Tauri Command
#[tauri::command]
fn return_string(word: String) -> String{
    // debugging message
    println!("The frontend sent us this argument {}", word);
    return word
}

fn main() {
  tauri::Builder::default()
    // Register Command with Tauri App
    .invoke_handler(tauri::generate_handler![return_string])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

mod world;
mod fonts;
mod packages;

/// Compile Typst Project
#[tauri::command]
fn compile_typst() {

}
