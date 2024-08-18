use std::fmt;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum NoteError {
    #[error("Note with the same title already exists.")]
    NoteAlreadyExists,
    #[error("Invalid Title.")]
    InvalidTitle,
    #[error("Invalid Description.")]
    InvalidDescription,
    #[error("Note Does not exist.")]
    NoteDoesNotExist
}

#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("Failed to create note file at home directory.")]
    CreateNoteFile,
    #[error("Failed to update note file.")]
    UpdateNoteFile,
    #[error("Failed to read note file.")]
    ReadNoteFile,
    #[error("Note file does not exist in home directory.")]
    NoteFileDoesNotExist,
}