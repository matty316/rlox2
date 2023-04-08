use crate::token::{Token, TokenType, Literal};

pub struct Scanner<'a> {
    source: &'a [u8],
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub const fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            let b = self.advance();
 
            match b {
                b'(' => self.add_token(TokenType::LPAREN),
                b')' => self.add_token(TokenType::RPAREN),
                b'{' => self.add_token(TokenType::LBRACE),
                b'}' => self.add_token(TokenType::RBRACE),
                b',' => self.add_token(TokenType::COMMA),
                b'.' => self.add_token(TokenType::DOT),
                b'-' => self.add_token(TokenType::MINUS),
                b'+' => self.add_token(TokenType::PLUS),
                b';' => self.add_token(TokenType::SEMICOLON),
                b'*' => self.add_token(TokenType::STAR),
                b'!' => self.operator(TokenType::BANG_EQ, TokenType::BANG),
                b'=' => self.operator(TokenType::EQ_EQ, TokenType::EQ),
                b'<' => self.operator(TokenType::LT_EQ, TokenType::LT),
                b'>' => self.operator(TokenType::GT_EQ, TokenType::GT),
                _ => { 
                    eprintln!("illegal char")
                }
            };
        }

        self.add_token(TokenType::EOF);

        self.tokens.clone() 
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text_vec = self.source[self.start..self.current].to_vec();
        let text = String::from_utf8(text_vec).expect("couldn't build string");

        let lit: Literal;

        match literal {
            Some(l) => lit = l,
            None => lit = Literal::None,
        }

        self.tokens.push(Token {
            lexeme: text,
            token_type,
            literal: lit,
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let prev = self.current;
        self.current += 1;
        self.source[prev]
    }

    fn match_op(&mut self, expected: u8) -> bool {
        if self.is_at_end() { return false; }
        if self.source[self.current] != expected { return false; }

        self.current += 1;
        true
    }
    
    fn operator(&mut self, token_type_1: TokenType, token_type_2: TokenType) {
        let token_type: TokenType;
        if self.match_op(b'=') { 
            token_type = token_type_1 
        } else {      
            token_type = token_type_2 
        }
        self.add_token(token_type)
    }
}

#[test]
fn single_token() {
    let expected = vec![
        TokenType::LPAREN,
        TokenType::RPAREN,
        TokenType::LBRACE,
        TokenType::RBRACE,
        TokenType::COMMA,
        TokenType::DOT,
        TokenType::MINUS,
        TokenType::PLUS,
        TokenType::SEMICOLON,
        TokenType::STAR,
        TokenType::EOF,
    ];

    let source = "(){},.-+;*";
    check_token_types(source, expected);
}

#[test]
fn operators() {
    let expected = vec![
        TokenType::BANG,
        TokenType::BANG_EQ,
        TokenType::EQ,
        TokenType::EQ_EQ,
        TokenType::LT,
        TokenType::LT_EQ,
        TokenType::GT,
        TokenType::GT_EQ,
    ];

    let source = "
    ! !=
    = ==
    < <=
    > >=
    ";

    check_token_types(source, expected);
}

fn check_token_types(source: &str, expected: Vec<TokenType>) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();

    for (i, e) in expected.iter().enumerate() {
        assert_eq!(e, &tokens[i].token_type);
    }
}
