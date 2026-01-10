use colored::Colorize;


pub struct CErr{}

impl CErr{
    pub fn throw_error(message: &str) {
        eprintln!("{}{}", String::from("Err: ").red(), String::from(message).red().italic());
    }
}