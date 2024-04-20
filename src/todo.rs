use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct TodoForm {
    pub text: String,
}