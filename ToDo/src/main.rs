mod cerr; mod cli; mod todo_list; mod menu; mod task_builder; mod save_and_load;

use cli::Console;
use std::collections::HashMap;
use std::process::exit;
use task_builder::create_new_task;
use todo_list::{Task, ToDo};
use crate::cerr::CErr;

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
                Console::clear();
                exit(1);
            }
            _ => {
                Console::clear();
                CErr::throw_error("Invalid option!\n");
            }
        }
    }
}
