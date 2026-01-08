use std::io::Write;

pub struct Console {}

impl Console {
    pub fn clear() {
        print!("\x1Bc");
        std::io::stdout().flush().unwrap();
    }
}
