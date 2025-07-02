use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TaskStatus {
    Pending,
    Done,
}

impl Task {
    pub fn new(description: String) -> Self {
        Task {
            description,
            status: TaskStatus::Pending,
        }
    }
}
