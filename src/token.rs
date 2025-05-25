
pub enum TOKEN {
    IDENT(String),
    INT(String),
    LITERAL,
    EOF,
    // Operators
    ASSIGN,
    PLUS,
    ILLEGAL,
    SEMICOLON,
    COMMA,
    LPAREN,
    RPAREN,
    RBRACE,
    LBRACE
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: String,
    pub literal: String,
}



#[test]
fn TestNextToken() {

    let input: String = "=+(){},;";
    let mut lexar = Lexar::new(input);

    let test = vec![
        TOKEN::ASSIGN,
        TOKEN::PLUS,
        TOKEN::LPAREN,
        TOKEN::LBRACE,
        TOKEN::RBRACE,
        TOEKN::COMMA,
        TOKEN::SEMICOLON,
        TOKEN::EOF
    ];

    for token in test {
        let nextToken = lexar.NextToken();
        println!("expected {:?}, recieved {:?}", token, nextToken)
    }

}