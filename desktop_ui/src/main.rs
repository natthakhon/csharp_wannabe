// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn run_extraction(name: String) -> String {
    println!("Rust received: {}", name);
    // This is where you will eventually call your pgsql_provider!
    format!("Success! Hello {}, the Rust backend is working.", name)
}

fn main() {
  tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_extraction]) // Register it here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
