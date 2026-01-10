mod cerr;
mod cli;
mod todo_list;

use cli::Console;
use std::collections::HashMap;
use std::process::exit;
use task_builder::create_new_task;
use todo_list::{Task, ToDo};
use crate::cerr::CErr;

mod menu {
    use colored::{Colorize};
    use std::io::{stdin, Write};

    pub fn state_menu_points(){
        println!(
            "{}{}{}",
            String::from("|Save:      (5)|").black().on_green(),
            String::from("|Load:      (6)|").black().on_cyan(),
            String::from("|Exit:      (7)|").black().on_red()
        )
    }

    pub fn control_menu_points() -> String {
        println!(
            "\nControls: \n{}\n{}\n{}\n{}\n",
            String::from("|Add:         (1)|").cyan(),
            String::from("|Edit:        (2)|").cyan(),
            String::from("|Change Done: (3)|").cyan(),
            String::from("|Remove:      (4)|").red(),
        );
        let mut option: String = String::new();
        print!("Command: ");
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut option).unwrap();
        option.trim().to_string()
    }
}

mod save_and_load {
    use crate::todo_list::{Task, ToDo};
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader, BufWriter, Write};

    pub fn load() -> ToDo {
        let reader = BufReader::new(File::open("save.txt").unwrap());
        let mut task_list = HashMap::new();
        for line in reader.lines().filter_map(|l| l.ok()) {
            let parts: Vec<&str> = line.split(";").collect();
            task_list.insert(
                parts[0].to_string(),
                Task {
                    name: parts[0].to_string(),
                    description: parts[1].to_string(),
                    is_done: parts[2].parse().expect("Not able to open the save!"),
                },
            );
        }
        ToDo { task_list }
    }

    pub fn save(task_list: &HashMap<String, Task>) {
        let mut stream = BufWriter::new(File::create("save.txt").unwrap());
        for (key, task) in task_list {
            writeln!(stream, "{};{};{}", key, task.description, task.is_done).unwrap();
        }
        stream.flush().unwrap();
    }
}

mod task_builder {
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
}

fn main() {
    Console::clear();
    let task_list: HashMap<String, Task> = HashMap::new();
    let mut todo = ToDo { task_list };
    loop {
        menu::state_menu_points();
        todo.print_out_tasks();
        let option = menu::control_menu_points();
        match option.as_str() {
            "1" => {
                let task = create_new_task();
                todo.add_task(task);
            }
            "2" => {
                let name = task_builder::set_name();
                let description = task_builder::set_description();
                todo.edit_task(name, description);
            }
            "3" => {
                let name = task_builder::set_name();
                todo.change_to_done(name);
            }
            "4" => {
                let name = task_builder::set_name();
                todo.remove_task(&name);
            }
            "5" => {
                save_and_load::save(&todo.task_list);
                Console::clear();
            }
            "6" => {
                todo = save_and_load::load();
                Console::clear();
            }
            "7" => {
                exit(1);
            }
            _ => {
                Console::clear();
                CErr::throw_error("Invalid option!\n");
            }
        }
    }
}
