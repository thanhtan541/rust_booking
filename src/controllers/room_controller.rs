use rocket::{delete, get, patch, post};
use std::fs;

#[get("/rooms/<id>")]
pub fn show(id: u32) -> String {
    let file_path = "data/rooms.json";

    // Attempt to read the file content
    match fs::read_to_string(file_path) {
        Ok(content) => content,
        _ => "I'm am a ZZ TeaPot".to_string(),
    }
}

#[get("/rooms")]
pub fn list() -> String {
    let file_path = "data/rooms.json";

    // Attempt to read the file content
    match fs::read_to_string(file_path) {
        Ok(content) => content,
        _ => "I'm am a ZZ TeaPot".to_string(),
    }
}

#[post("/rooms")]
pub fn add() -> String {
    let file_path = "data/rooms.json";

    // Attempt to read the file content
    match fs::read_to_string(file_path) {
        Ok(content) => content,
        _ => "I'm am a ZZ TeaPot".to_string(),
    }
}

#[patch("/rooms/<id>")]
pub fn patch(id: u32) -> String {
    let file_path = "data/rooms.json";

    // Attempt to read the file content
    match fs::read_to_string(file_path) {
        Ok(content) => content,
        _ => "I'm am a ZZ TeaPot".to_string(),
    }
}

#[delete("/rooms/<id>")]
pub fn delete(id: u32) -> String {
    let file_path = "data/rooms.json";

    // Attempt to read the file content
    match fs::read_to_string(file_path) {
        Ok(content) => content,
        _ => "I'm am a ZZ TeaPot".to_string(),
    }
}
