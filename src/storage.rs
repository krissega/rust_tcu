use crate::task::Task;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const FILE_PATH: &str = "tasks.json";

pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let data = serde_json::to_string_pretty(tasks).expect("Error serializando tareas");
    let mut file = fs::File::create(FILE_PATH)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(FILE_PATH)?;
    let tasks: Vec<Task> = serde_json::from_str(&data).expect("Error deserializando tareas");
    Ok(tasks)
}
