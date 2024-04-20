use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Todo {
    pub id: usize,
    pub text: String,
}

#[derive(Deserialize)]
pub struct TodoForm {
    pub text: String,
}