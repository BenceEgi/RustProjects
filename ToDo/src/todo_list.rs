use crate::cerr;
use crate::cli::Console;
use colored::Colorize;
use std::collections::HashMap;

//TASK
pub struct Task {
    pub name: String,
    pub description: String,
    pub is_done: bool,
}

impl Task {
    pub fn change_is_done_param(&mut self) {
        self.is_done = !self.is_done;
    }
}

//TODOLIST
pub struct ToDo {
    pub task_list: HashMap<String, Task>,
}

impl<'a> ToDo {
    pub fn print_out_tasks(&self) {
        println!("{}:\n", String::from("Tasks").yellow().bold().underline());
        if self.task_list.len() == 0 {
            println!(
                "{}",
                String::from(" You don't have any tasks!").red().bold()
            );
            return;
        };
        for (name, task) in &self.task_list {
            println!(
                " {}:\n  Description: {}\n  Done: {}\n",
                name.purple(),
                task.description.blue(),
                if task.is_done {
                    String::from("✅")
                } else {
                    String::from("❌")
                }
            );
        }
    }

    pub fn add_task(&mut self, new_task: Task) {
        let message = &format!("Task with the name '{}' already exist!", new_task.name);
        if let Some(_task) =  self.does_it_exist(&new_task.name) {
            Console::clear();
            cerr::throw_error(message);
        }
        else {
            self.task_list.insert(new_task.name.clone(), new_task);
            Console::clear();
        };
    }

    pub fn remove_task(&mut self, name: &String) {
        match self.task_list.get(name) {
            Some(_) => {
                Console::clear();
                cerr::throw_error("The task list is empty!\n");
            }
            None => {
                self.task_list.remove(name);
                Console::clear()
            }
        }
    }

    pub fn edit_task(&mut self, name: String, new_description: String) {
        self.task_list
            .entry(name)
            .and_modify(|task: &mut Task| task.description = new_description);
    }

    pub fn change_to_done(&mut self, name: String) {
        self.task_list
            .entry(name)
            .and_modify(|task| task.change_is_done_param());
    }

    pub fn does_it_exist(&self, name: &String) -> Option<&Task> {
        self.task_list.get(name)
    }
}
