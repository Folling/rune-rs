use std::collections::HashSet;

use crate::transpiler::ParseError;
use crate::Token;

pub struct Lexer<'a> {
    content: &'a str,
    idx: usize,
    specials: HashSet<&'static str>,
    current_token: Option<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        Lexer {
            content,
            idx: 0,
            specials: [
                "<", ">", "=", "==", "!=", "<=>", "<=", ">=", "<<", ">>", "<<=", ">>=", "&", "|", "^", "&&", "||", "^^", "&=", "|=", "^=",
                "&&=", "||=", "^^=", "!", "-", "+", "/", "*", "&", "**", "//", "+=", "-=", "*=", "/=", "%=", "**=", "//=", "$", "\"", "\'",
                "#", ",", ".", ":", "::", "->", "(", ")", "[", "]", "{", "}", "?",
            ]
            .into_iter()
            .collect(),
            current_token: None,
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        match self.content.as_bytes().get(self.idx) {
            Some(byte) => {
                let count = byte.leading_ones().max(1) as usize;

                // no need to check the upper bound here, rust guarantees valid UTF8
                let ret = unsafe {
                    char::from_u32_unchecked(
                        self.content.as_bytes()[self.idx..self.idx + count]
                            .into_iter()
                            .zip([
                                ((count.saturating_sub(1)) * 6, if count == 1 { 0b1111111 } else { 0b11111 }),
                                ((count.saturating_sub(2)) * 6, 0b111111),
                                ((count.saturating_sub(3)) * 6, 0b111111),
                                (0, 0b111111),
                            ])
                            .fold(0, |acc, (v, (s, b))| acc | (((*v & b) as u32) << s)),
                    )
                };

                Some(ret)
            }
            None => None,
        }
    }

    fn consume_char(&mut self, c: char) {
        self.idx += c.len_utf8();
    }

    fn predicate<F: Fn(char) -> bool>(&mut self, f: F) -> bool {
        if let Some(next) = self.peek_char() {
            return if f(next) {
                self.consume_char(next);
                true
            } else {
                false
            };
        }

        false
    }

    fn next(&mut self) -> Option<Token<'a>> {
        while self.predicate(char::is_whitespace) {}

        let start = self.idx;

        if let Some(next) = self.peek_char() {
            self.consume_char(next);

            if next.is_alphanumeric() {
                while self.predicate(char::is_alphanumeric) {}

                Some(Token::Textual(unsafe { self.content.get_unchecked(start..self.idx) }))
            } else {
                match self.peek_char() {
                    Some(c)
                        if !c.is_whitespace()
                            && !c.is_alphanumeric()
                            && self
                                .specials
                                .contains(unsafe { self.content.get_unchecked(start..=start + c.len_utf8()) }) =>
                    {
                        self.consume_char(c)
                    }
                    _ => {}
                }

                Some(Token::Special(unsafe { self.content.get_unchecked(start..self.idx) }))
            }
        } else {
            None
        }
    }

    pub fn cur_next(&mut self) -> Option<Token<'a>> {
        let ret = self.current_token;

        self.current_token = self.next();

        ret
    }

    pub fn next_cur(&mut self) -> Option<Token<'a>> {
        self.current_token = self.next();

        self.current_token
    }

    pub fn cur(&self) -> Option<Token<'a>> {
        self.current_token
    }
}
