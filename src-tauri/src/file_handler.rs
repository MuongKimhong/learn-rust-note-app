use std::{env, path, fs};
use serde_json::{to_writer, from_reader, json};

use crate::notes::Note;
use crate::errors::FileError;

pub fn note_file_exists() -> bool {
    if let Ok(home_path) = env::var("HOME") {
        let note_file_path = path::Path::new(&home_path).join("mynote.json");
        
        if note_file_path.exists() {
            return true;
        } 
    }    
    return false;
}

fn create_note_file() -> Result<(), FileError> {
    let mut note_file_path = String::new();

    if let Ok(home_path) = env::var("HOME") {
        let file_path = path::Path::new(&home_path).join("mynote.json"); 
        note_file_path = file_path.to_string_lossy().to_string();
    }
    let mut file = match fs::File::create(note_file_path) {
        Ok(f) => f,
        Err(_) => return Err(FileError::CreateNoteFile)
    };
    let empty_array = json!([]);
    match serde_json::to_writer(&mut file, &empty_array) {
        Ok(_) => Ok(()),
        Err(_) => Err(FileError::CreateNoteFile),
    }
}

// when program start, mynote.json will be created in home directory
// if it doesn't exist
pub fn initialize_note_file() -> Result<(), FileError> {
    if note_file_exists() {
        return Ok(());
    }
    create_note_file()?;
    Ok(())
}

pub fn read_note_file() -> Result<Vec<Note>, FileError> {
    if !note_file_exists() {
        return Err(FileError::NoteFileDoesNotExist);
    }
    match env::var("HOME") {
        Ok(home_path) => {
            let file_path = path::Path::new(&home_path).join("mynote.json"); 

            let file = match fs::File::open(&file_path) {
                Ok(f) => f,
                Err(_) => return Err(FileError::NoteFileDoesNotExist),
            };
            match from_reader(file) {
                Ok(notes) => return Ok(notes),
                Err(_) => return Err(FileError::ReadNoteFile),
            }
        },
        Err(_) => return Err(FileError::NoteFileDoesNotExist),
    }
}

pub fn update_note_file(all_notes: &Vec<Note>) -> Result<(), FileError> {
    if !note_file_exists() {
        return Err(FileError::NoteFileDoesNotExist);
    }
    match env::var("HOME") {
        Ok(home_path) => {
            let file_path = path::Path::new(&home_path).join("mynote.json"); 

            let file = match fs::File::create(&file_path) {
                Ok(f) => f,
                Err(_) => return Err(FileError::NoteFileDoesNotExist),
            };
            match to_writer(file, all_notes) {
                Ok(_) => return Ok(()),
                Err(_) => return Err(FileError::UpdateNoteFile),
            }
        },
        Err(_) => return Err(FileError::NoteFileDoesNotExist),
    }
}
