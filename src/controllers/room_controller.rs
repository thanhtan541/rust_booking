use std::fs;
use rocket::{get};

#[get("/api/rooms")]
pub fn fun() -> String {
    let file_path = "data/rooms.json";

    // Attempt to read the file content
    match fs::read_to_string(file_path) {
        Ok(content) => {
            content
        }
        _ => {
            "I'm am a ZZ TeaPot".to_string()
        }
    }
}

