#![allow(unused)]

use core::panic;
use std::path::Iter;
use token::Token;
use token::TokenKind;

mod token;

pub struct Lexer<'a> {
    input: &'a str,
    /// current position in input (points to current char)
    position: usize,
    /// current reading position in input (after current char)
    read_position: usize,
    /// current char under examination
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut t = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        t.read_char();
        t
    }
}

impl<'a> Lexer<'a> {
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn consume_char(&mut self) -> String {
        let ch = self.ch;
        self.read_char();
        ch.to_string()
    }

    pub fn read_ident(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        self.input[position..self.position].to_owned()
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].to_owned()
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn is_letter(&self) -> bool {
        self.ch.is_alphabetic() || self.ch == '_'
    }

    pub fn is_digit(&self) -> bool {
        self.ch.is_ascii_digit()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if self.ch == '\0' {
            return None;
        }

        let token = match self.ch {
            ';' => Token::new(TokenKind::Semi, self.consume_char()),
            ',' => Token::new(TokenKind::Comma, self.consume_char()),
            '.' => Token::new(TokenKind::Dot, self.consume_char()),
            '(' => Token::new(TokenKind::OpenParen, self.consume_char()),
            ')' => Token::new(TokenKind::CloseParen, self.consume_char()),
            '{' => Token::new(TokenKind::OpenBrace, self.consume_char()),
            '}' => Token::new(TokenKind::CloseBrace, self.consume_char()),
            '[' => Token::new(TokenKind::OpenBracket, self.consume_char()),
            ']' => Token::new(TokenKind::CloseBracket, self.consume_char()),
            '@' => Token::new(TokenKind::At, self.consume_char()),
            '#' => Token::new(TokenKind::Pound, self.consume_char()),
            '~' => Token::new(TokenKind::Tilde, self.consume_char()),
            '!' => Token::new(TokenKind::Bang, self.consume_char()),
            '=' => Token::new(TokenKind::Eq, self.consume_char()),
            '+' => Token::new(TokenKind::Plus, self.consume_char()),
            '-' => Token::new(TokenKind::Minus, self.consume_char()),
            '*' => Token::new(TokenKind::Star, self.consume_char()),
            '/' => Token::new(TokenKind::Slash, self.consume_char()),
            '<' => Token::new(TokenKind::Lt, self.consume_char()),
            '>' => Token::new(TokenKind::Gt, self.consume_char()),
            '&' => Token::new(TokenKind::And, self.consume_char()),
            '|' => Token::new(TokenKind::Or, self.consume_char()),
            '^' => Token::new(TokenKind::Caret, self.consume_char()),
            ':' => Token::new(TokenKind::Colon, self.consume_char()),
            '?' => Token::new(TokenKind::Question, self.consume_char()),
            '$' => Token::new(TokenKind::Dollar, self.consume_char()),
            '%' => Token::new(TokenKind::Percent, self.consume_char()),
            _ if self.is_letter() => Token::new(TokenKind::Ident, self.read_ident()),
            _ if self.is_digit() => Token::new(
                TokenKind::Literal {
                    kind: token::LiteralKind::Int,
                },
                self.read_number(),
            ),
            _ => Token::new(TokenKind::Unknown, self.consume_char()),
        };

        Some(token)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_char_token() {
        let input = r#"
            ;,.(){}[]@#~!=+-*/<>&|^:?$%
        "#;
        let mut tokenizer = Lexer::new(input);
        let expected = vec![
            Token::new(TokenKind::Semi, ";".to_owned()),
            Token::new(TokenKind::Comma, ",".to_owned()),
            Token::new(TokenKind::Dot, ".".to_owned()),
            Token::new(TokenKind::OpenParen, "(".to_owned()),
            Token::new(TokenKind::CloseParen, ")".to_owned()),
            Token::new(TokenKind::OpenBrace, "{".to_owned()),
            Token::new(TokenKind::CloseBrace, "}".to_owned()),
            Token::new(TokenKind::OpenBracket, "[".to_owned()),
            Token::new(TokenKind::CloseBracket, "]".to_owned()),
            Token::new(TokenKind::At, "@".to_owned()),
            Token::new(TokenKind::Pound, "#".to_owned()),
            Token::new(TokenKind::Tilde, "~".to_owned()),
            Token::new(TokenKind::Bang, "!".to_owned()),
            Token::new(TokenKind::Eq, "=".to_owned()),
            Token::new(TokenKind::Plus, "+".to_owned()),
            Token::new(TokenKind::Minus, "-".to_owned()),
            Token::new(TokenKind::Star, "*".to_owned()),
            Token::new(TokenKind::Slash, "/".to_owned()),
            Token::new(TokenKind::Lt, "<".to_owned()),
            Token::new(TokenKind::Gt, ">".to_owned()),
            Token::new(TokenKind::And, "&".to_owned()),
            Token::new(TokenKind::Or, "|".to_owned()),
            Token::new(TokenKind::Caret, "^".to_owned()),
            Token::new(TokenKind::Colon, ":".to_owned()),
            Token::new(TokenKind::Question, "?".to_owned()),
            Token::new(TokenKind::Dollar, "$".to_owned()),
            Token::new(TokenKind::Percent, "%".to_owned()),
        ];
        for e in expected {
            let next = tokenizer.next();
            assert_eq!(next.unwrap(), e);
        }
    }

    #[test]
    fn test_next_token() {
        let input = r#"
            let five = 5;
        "#;
        let mut lexer = Lexer::new(input);
        let expected = vec![
            Token::new(TokenKind::Ident, "let".to_owned()),
            Token::new(TokenKind::Ident, "five".to_owned()),
            Token::new(TokenKind::Eq, "=".to_owned()),
            Token::new(
                TokenKind::Literal {
                    kind: token::LiteralKind::Int,
                },
                "5".to_owned(),
            ),
            Token::new(TokenKind::Semi, ";".to_owned()),
        ];
        for e in expected {
            let next = lexer.next();
            assert_eq!(next.unwrap(), e);
        }
    }

    #[test]
    fn test_fn_declaration() {
        let input = r#"
            let five = 5.0;
            let ten = 10;
            let add = fn(x, y) {
                return x + y;
            };
            let result = add(five, ten);
        "#;
        let mut lexer = Lexer::new(input);
        let expected = vec![
            Token::new(TokenKind::Ident, "let".to_owned()),
            Token::new(TokenKind::Ident, "five".to_owned()),
            Token::new(TokenKind::Eq, "=".to_owned()),
            Token::new(
                TokenKind::Literal {
                    kind: token::LiteralKind::Int,
                },
                "5".to_owned(),
            ),
            Token::new(TokenKind::Dot, ".".to_owned()),
            Token::new(
                TokenKind::Literal {
                    kind: token::LiteralKind::Int,
                },
                "0".to_owned(),
            ),
            Token::new(TokenKind::Semi, ";".to_owned()),
            Token::new(TokenKind::Ident, "let".to_owned()),
            Token::new(TokenKind::Ident, "ten".to_owned()),
            Token::new(TokenKind::Eq, "=".to_owned()),
            Token::new(
                TokenKind::Literal {
                    kind: token::LiteralKind::Int,
                },
                "10".to_owned(),
            ),
            Token::new(TokenKind::Semi, ";".to_owned()),
            Token::new(TokenKind::Ident, "let".to_owned()),
            Token::new(TokenKind::Ident, "add".to_owned()),
            Token::new(TokenKind::Eq, "=".to_owned()),
            Token::new(TokenKind::Ident, "fn".to_owned()),
            Token::new(TokenKind::OpenParen, "(".to_owned()),
            Token::new(TokenKind::Ident, "x".to_owned()),
            Token::new(TokenKind::Comma, ",".to_owned()),
            Token::new(TokenKind::Ident, "y".to_owned()),
            Token::new(TokenKind::CloseParen, ")".to_owned()),
            Token::new(TokenKind::OpenBrace, "{".to_owned()),
            Token::new(TokenKind::Ident, "return".to_owned()),
            Token::new(TokenKind::Ident, "x".to_owned()),
            Token::new(TokenKind::Plus, "+".to_owned()),
            Token::new(TokenKind::Ident, "y".to_owned()),
            Token::new(TokenKind::Semi, ";".to_owned()),
            Token::new(TokenKind::CloseBrace, "}".to_owned()),
            Token::new(TokenKind::Semi, ";".to_owned()),
            Token::new(TokenKind::Ident, "let".to_owned()),
            Token::new(TokenKind::Ident, "result".to_owned()),
            Token::new(TokenKind::Eq, "=".to_owned()),
            Token::new(TokenKind::Ident, "add".to_owned()),
            Token::new(TokenKind::OpenParen, "(".to_owned()),
            Token::new(TokenKind::Ident, "five".to_owned()),
            Token::new(TokenKind::Comma, ",".to_owned()),
            Token::new(TokenKind::Ident, "ten".to_owned()),
            Token::new(TokenKind::CloseParen, ")".to_owned()),
            Token::new(TokenKind::Semi, ";".to_owned()),
        ];

        for e in expected {
            let next = lexer.next();
            assert_eq!(next.unwrap(), e);
        }
    }

    #[test]
    fn test_while_loop() {
        let input = r#"
            let i = 0;
            while (i < 10) {
                i = i + 1;
            }
        "#;
        let mut lexer = Lexer::new(input);
        let expected = vec![
            Token::new(TokenKind::Ident, "let".to_owned()),
            Token::new(TokenKind::Ident, "i".to_owned()),
            Token::new(TokenKind::Eq, "=".to_owned()),
            Token::new(
                TokenKind::Literal {
                    kind: token::LiteralKind::Int,
                },
                "0".to_owned(),
            ),
            Token::new(TokenKind::Semi, ";".to_owned()),
            Token::new(TokenKind::Ident, "while".to_owned()),
            Token::new(TokenKind::OpenParen, "(".to_owned()),
            Token::new(TokenKind::Ident, "i".to_owned()),
            Token::new(TokenKind::Lt, "<".to_owned()),
            Token::new(
                TokenKind::Literal {
                    kind: token::LiteralKind::Int,
                },
                "10".to_owned(),
            ),
            Token::new(TokenKind::CloseParen, ")".to_owned()),
            Token::new(TokenKind::OpenBrace, "{".to_owned()),
            Token::new(TokenKind::Ident, "i".to_owned()),
            Token::new(TokenKind::Eq, "=".to_owned()),
            Token::new(TokenKind::Ident, "i".to_owned()),
            Token::new(TokenKind::Plus, "+".to_owned()),
            Token::new(
                TokenKind::Literal {
                    kind: token::LiteralKind::Int,
                },
                "1".to_owned(),
            ),
            Token::new(TokenKind::Semi, ";".to_owned()),
            Token::new(TokenKind::CloseBrace, "}".to_owned()),
        ];

        for e in expected {
            let next = lexer.next();
            assert_eq!(next.unwrap(), e);
        }
    }
}
