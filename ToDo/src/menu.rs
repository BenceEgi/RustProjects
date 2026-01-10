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