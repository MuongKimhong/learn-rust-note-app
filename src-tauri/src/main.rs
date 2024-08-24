// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_handler;
mod commands;
mod errors;
mod notes;
mod test_commands;

use std::sync::Arc;
use std::sync::Mutex;
use crate::notes::Note;


type AppState = Arc<Mutex<Vec<Note>>>;

fn main() {
  let app_state = Arc::new(Mutex::new(Vec::new()));

  // read notes from file
  {
    if file_handler::note_file_exists() {
      let mut all_notes = app_state.lock().unwrap();

      *all_notes = match file_handler::read_note_file() {
        Ok(notes) => notes,
        Err(_) => std::process::exit(1),
      }
    }
  }

  tauri::Builder::default()
    .manage(app_state)
    .invoke_handler(tauri::generate_handler![
      commands::initialize_note_file, 
      commands::add_new_note, 
      commands::get_all_notes, 
      commands::delete_note, 
      commands::update_title,
      commands::update_description,
      commands::search_notes,
      commands::exit_app
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
