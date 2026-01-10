use crate::todo_list::{Task, ToDo};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn load() -> ToDo {
    let file_result = File::open("save.txt");
    let file;
    match file_result {
        Ok(f) => {file = f},
        Err(_e) => { file = File::create_new("./save.txt").unwrap() }
    }
    let reader = BufReader::new(file);
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