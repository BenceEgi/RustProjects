use std::fs::File;
use std::io::{BufReader, Read};

pub struct Lexer<'a>{
    pub file: &'a str,
}

impl<'a> Lexer<'a> {
    fn read_file(&self) -> Vec<char>{
        let mut buff_read = BufReader::new(File::open(&self.file).unwrap());
        let mut buf = Vec::new();
        buff_read.read_to_end(&mut buf).unwrap();
        let chr_vec: Vec<char> = buf.iter().map(|b| {return *b as char;}).collect();
        chr_vec
    }
}

struct LexerSpec{
    start: i32,
    current: i32,
    line: i32
}

impl LexerSpec {
    pub fn end_of_file(){

    }
}