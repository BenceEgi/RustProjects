use std::io::Write;

pub struct Console {}

impl Console {
    pub fn clear(){
        print!("{}[2J", 27 as char);
        std::io::stdout().flush().unwrap();
    }
}