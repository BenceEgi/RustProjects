mod token;
mod lexer;

use lexer::Lexer;

fn main() {
    let lexer = Lexer{file: "./test.shl"};
}