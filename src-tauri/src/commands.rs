use tauri::State;
use crate::AppState;
use crate::file_handler;
use crate::notes::Note;

#[tauri::command]
pub fn initialize_note_file() -> Result<String, String> {
  match file_handler::initialize_note_file() {
    Ok(_) => return Ok(String::from("success")),
    Err(e) => {
      return Err(format!("{:?}", e));
    } 
  }
}

#[tauri::command]
pub fn get_all_notes(state: State<AppState>) -> Vec<Note> {
  let all_notes = state.all_notes.lock().unwrap();
  all_notes.clone()
}

#[tauri::command]
pub fn add_new_note(state: State<AppState>, title: String, description: String) -> Result<String, String> {
  let mut all_notes = state.all_notes.lock().unwrap();

  match Note::new(&mut all_notes, &title, &description) {
    Ok(_) => {
      match file_handler::update_note_file(&all_notes) {
        Ok(_) => Ok(String::from("success")),
        Err(e) => Err(format!("{:?}", e)),
      }
    },
    Err(e) => Err(format!("{:?}", e)),
  }
}

#[tauri::command]
pub fn delete_note(state: State<AppState>, note_id: u8) -> Result<String, String> {
  let mut all_notes = state.all_notes.lock().unwrap();

  match Note::delete_note(&mut all_notes, note_id) {
    Ok(_) => {
      match file_handler::update_note_file(&all_notes) {
        Ok(_) => Ok(String::from("success")),
        Err(e) => Err(format!("{:?}", e)),
      }
    },
    Err(e) => Err(format!("{:?}", e)),
  }
}

#[tauri::command]
pub fn update_title(state: State<AppState>, title: String, note_id: u8) -> Result<String, String> {
  let mut all_notes = state.all_notes.lock().unwrap();

  for note in all_notes.iter_mut() {
    if note.id == note_id {
      note.update_title(title);
      break;
    }
  }
  match file_handler::update_note_file(&all_notes) {
    Ok(_) => Ok(String::from("success")),
    Err(e) => Err(format!("{:?}", e)),
  }
}

#[tauri::command]
pub fn update_description(state: State<AppState>, description: String, note_id: u8) -> Result<String, String> {
  let mut all_notes = state.all_notes.lock().unwrap();

  for note in all_notes.iter_mut() {
    if note.id == note_id {
      note.update_description(description);
      break;
    }
  }
  match file_handler::update_note_file(&all_notes) {
    Ok(_) => Ok(String::from("success")),
    Err(e) => Err(format!("{:?}", e)),
  }
}

#[tauri::command]
pub fn search_notes(state: State<AppState>, search_text: &str) -> Vec<Note> {
  let all_notes = state.all_notes.lock().unwrap();
  let results = Note::search_notes(&all_notes, search_text);
  results
}

#[tauri::command]
pub fn exit_app() {
  std::process::exit(1);
}