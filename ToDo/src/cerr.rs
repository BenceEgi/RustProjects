use colored::Colorize;

pub fn throw_error(message: &str) {
    eprintln!("{}", String::from(message).red().italic());
}