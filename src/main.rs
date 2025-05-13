// src/main.rs
use ardbms::file_manager;

fn main() {
    // Correct way to create a new FileManager instance
    let manager = file_manager::FileManager::new();
    manager.demo();
}
