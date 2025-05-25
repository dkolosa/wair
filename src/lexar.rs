use crate::token::{Token, TOKEN};

pub struct Lexer{

    pub input: Vec<u8>,
    pub posiiton:usize,
    pub read_position:usize,
    pub ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer { 
            input: input.into_bytes(), 
            posiiton: 0, 
            read_position: 0, 
            ch: 0 
        };

        l.read_char();
        return l;

    
    }

    pub fn read_char(&mut self)  {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.posiiton = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> TOKEN{
        

        let tok: TOKEN = match self.ch {
            b'=' => TOKEN::ASSIGN,
            b';' => TOKEN::SEMICOLON,
            b',' => TOKEN::COMMA,
            b'(' => TOKEN::LPAREN,
            b')' => TOKEN::RPAREN,
            b'{' => TOKEN::LBRACE,
            b'}' => TOKEN::RBRACE,
            b'+' => TOKEN::PLUS,
            b'\0' => TOKEN::EOF,
            _ => TOKEN::LITERAL
        };

        self.read_char();
        return tok;
    }

}