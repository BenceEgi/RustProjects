use crate::todo_list::Task;
use std::io::{stdin, Write};

pub fn set_name() -> String {
    let mut title: String = String::new();
    print!("Task title: ");
    _ = std::io::stdout().flush();
    _ = stdin().read_line(&mut title);
    title.trim().to_string()
}

pub fn set_description() -> String {
    let mut description: String = String::new();
    print!("Task description: ");
    std::io::stdout().flush().unwrap();
    stdin().read_line(&mut description).unwrap();
    description.trim().to_string()
}

pub fn create_new_task() -> Task {
    Task {
        name: set_name(),
        description: set_description(),
        is_done: false,
    }
}