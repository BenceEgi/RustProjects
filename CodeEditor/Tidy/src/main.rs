use std::io;
use std::io::Read;

struct Key{
    byte: u8,
    chr: char
}

struct KeyReader{}

impl KeyReader{
    pub fn try_read_key(out: &mut Key) -> bool{
        let mut byte_key= [0];
        match io::stdin().read_exact(&mut byte_key) {
            Ok(_val) => {*out = Key{ byte: byte_key[0], chr: byte_key[0] as char }; return true;},
            Err(_err) => eprintln!("Err: something went wrong!")
        };
        false
    }
}

fn main() {
    let mut key: Key = Key{byte:0, chr:' '};
    while (KeyReader::try_read_key(&mut key) && key.chr != 'q') {
        
    }
    println!("Exited");
}