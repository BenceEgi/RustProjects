pub struct Token<'a>
{
    token_type: Types<'a>
}

pub enum Types<'a>{
    EQUAL,
    MINUS,
    PLUS,
    MULTI,
    DIV,
    LPAREN,
    RPAREN,
    SEMI,

    // LITERALS
    ID(&'a str),
    INT32(i32),
    FLOAT(f32),
    STRING(String),
    EOF
}