use serde::{Serialize, Deserialize};
use chrono::offset::Local;
use chrono::DateTime;

use crate::errors::NoteError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    pub id: u8,
    pub title: String,
    pub description: String,
    pub created_at: String
}

impl Note {
    fn find_lastest_id(all_notes: &Vec<Note>) -> u8 {
        let mut note_id: u8 = 1;

        if all_notes.len() >= 1 {
            let last_note: &Note = all_notes.last().unwrap(); 
            note_id = last_note.id + 1;
        }
        note_id
    } 
    fn note_title_exists(all_notes: &Vec<Note>, title: &String) -> bool {
        for note in all_notes {
            if note.title == *title {
                return true;
            }
        }
        return false;
    }
    pub fn new(all_notes: &mut Vec<Note>, title: &String, description: &String) -> Result<(), NoteError> {
        if title.is_empty() {
            return Err(NoteError::InvalidTitle);
        }
        if description.is_empty() {
            return Err(NoteError::InvalidDescription);
        }
        if Self::note_title_exists(all_notes, title) {
            return Err(NoteError::NoteAlreadyExists);
        }

        let new_note_id: u8 = Self::find_lastest_id(all_notes);

        let now: DateTime<Local> = Local::now();

        let new_note = Note {
            id: new_note_id,
            title: title.clone(),
            description: description.clone(),
            created_at: now.format("%A %d/%B/%Y, %I:%M %P").to_string(),
        };
        all_notes.push(new_note);
        Ok(())
    }
    pub fn search_notes(all_notes: &Vec<Note>, search_text: &str) -> Vec<Note> {
        let mut results: Vec<Note> = Vec::new();

        for note in all_notes {
            if note.title.contains(search_text) {
                results.push(note.clone());
            }
        }
        results
    }
    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title; 
    }
    pub fn update_description(&mut self, new_description: String) {
        self.description = new_description;
    }
    pub fn delete_note(all_notes: &mut Vec<Note>, note_id: u8) -> Result<(), NoteError> {
        if all_notes.len() == 0 {
            return Err(NoteError::NoteDoesNotExist);
        }
        for (index, note) in all_notes.iter().enumerate() {
            if note.id == note_id {
                all_notes.remove(index);
                return Ok(());
            }
        }
        return Err(NoteError::NoteDoesNotExist);
    }
}