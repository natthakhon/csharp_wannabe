// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use my_models::MyModel;
use chrono::{DateTime, Utc};
use pgsql_provider::PgSqlRandomRepo;
use repository_traits::my_traits::RandomRepo;

#[tauri::command]
fn run_extraction(name: String) -> String {
    println!("Rust received: {}", name);
    // This is where you will eventually call your pgsql_provider!
    format!("Success! Hello {}, the Rust backend is working.", name)
}

#[tauri::command]
async fn insert(mystring: String,myint:i32) -> Result<String, String> {
  let created_at = Utc::now();
  let model = MyModel::new(&mystring, myint, created_at);
  let db_url = "postgres://postgres:postgres@localhost:5432/rust";
  let repo = PgSqlRandomRepo::new(db_url).await
        .map_err(|e| e.to_string())?;
  repo.save(&model).await.map_err(|e| e.to_string())?;
  Ok(format!("Model created for {} at {}", model.my_string, model.created_at))
}


fn main() {
  tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_extraction,insert]) // Register it here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
