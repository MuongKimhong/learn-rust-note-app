#![cfg(test)]

use tauri::{InvokePayload, Manager};
use tauri::test::mock_builder;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::Write;
use serial_test::serial;

use crate::file_handler;
use crate::commands;
use crate::errors;
use crate::notes::Note;

fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
    let app_state = Arc::new(Mutex::new(Vec::<Note>::new()));
    builder
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
        .build(tauri::generate_context!("tauri.conf.json")) // path to tauri.conf.json relative to root directory of rust
        .expect("failed to build app")
}

fn invoke_payload_without_args(cmd_name: &str) -> InvokePayload {
    InvokePayload {
        cmd: cmd_name.into(),
        tauri_module: None,
        callback: tauri::api::ipc::CallbackFn(0),
        error: tauri::api::ipc::CallbackFn(1),
        inner: serde_json::Value::Null,
        invoke_key: Some(tauri::test::INVOKE_KEY.into()),
    }
}

fn invoke_payload_with_args(cmd_name: &str, args: String) -> InvokePayload {
    InvokePayload {
        cmd: cmd_name.into(),
        tauri_module: None,
        callback: tauri::api::ipc::CallbackFn(0),
        error: tauri::api::ipc::CallbackFn(1),
        inner: serde_json::from_str(&args).unwrap(),
        invoke_key: Some(tauri::test::INVOKE_KEY.into()),
    }
}

#[test]
fn test_initialize_note_file_command() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_without_args("initialize_note_file"),
        Ok("success".to_string()) // assert, Ok or Err
    );
}

#[test]
fn test_get_all_notes_command() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    let expected_response: Vec<Note> = Vec::new();

    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_without_args("get_all_notes"),
        Ok(expected_response)
    );
}

#[test]
fn test_add_new_note_command_success() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    let title = "test add note title".to_string();
    let description = "test add note description".to_string();
    let json_data = format!(
        r#"{{"title": "{}","description": "{}"}}"#,
        title,
        description
    );
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("add_new_note", json_data),
        Ok("success".to_string())
    );
}

#[test]
fn test_add_new_note_command_invalid_title() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    let title = "".to_string();
    let description = "test add note description".to_string();
    let json_data = format!(
        r#"{{"title": "{}","description": "{}"}}"#,
        title,
        description
    );
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("add_new_note", json_data),
        Err("InvalidTitle".to_string())
    );
}

#[test]
fn test_add_new_note_command_invalid_description() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    let title = "test add note title".to_string();
    let description = "".to_string();
    let json_data = format!(
        r#"{{"title": "{}","description": "{}"}}"#,
        title,
        description
    );
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("add_new_note", json_data),
        Err("InvalidDescription".to_string())
    );
}

#[test]
fn test_add_new_note_command_title_exists() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    let title = "title exist".to_string();
    let description = "test add note description".to_string();
    let json_data = format!(
        r#"{{"title": "{}","description": "{}"}}"#,
        title,
        description
    );
    // add one note for comparison
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("add_new_note", json_data.clone()),
        Ok("success".to_string())
    );
    // compare with previous note
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("add_new_note", json_data),
        Err("NoteAlreadyExists".to_string())
    );
}

#[test]
fn test_delete_note_command_success() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    let title = "test title".to_string();
    let description = "test description".to_string();
    let json_data = format!(
        r#"{{"title": "{}","description": "{}"}}"#,
        title,
        description
    );
    // add one note for delete command
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("add_new_note", json_data.clone()),
        Ok("success".to_string())
    );

    let note_id = r#"{"noteId": 1}"#;
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("delete_note", note_id.to_string()),
        Ok("success".to_string())
    );
}

#[test]
fn test_delete_note_command_fail() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    let note_id = r#"{"noteId": 20}"#;
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("delete_note", note_id.to_string()),
        Err("NoteDoesNotExist".to_string())
    );
}

#[test]
fn test_update_title_command() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    let title = "test title".to_string();
    let description = "test description".to_string();
    let json_data = format!(
        r#"{{"title": "{}","description": "{}"}}"#,
        title,
        description
    );
    // add one note for update title command
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("add_new_note", json_data),
        Ok("success".to_string())
    );

    let update_required_args = r#"{"title": "new title", "noteId": 1}"#;
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("update_title", update_required_args.to_string()),
        Ok("success".to_string())
    );
}

#[test]
fn test_update_description_command() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    let title = "test title".to_string();
    let description = "test description".to_string();
    let json_data = format!(
        r#"{{"title": "{}","description": "{}"}}"#,
        title,
        description
    );
    // add one note for update title command
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("add_new_note", json_data),
        Ok("success".to_string())
    );

    let update_required_args = r#"{"description": "new description", "noteId": 1}"#;
    tauri::test::assert_ipc_response(
        &window,
        invoke_payload_with_args("update_description", update_required_args.to_string()),
        Ok("success".to_string())
    );
}
