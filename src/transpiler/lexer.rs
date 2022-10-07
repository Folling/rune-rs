use std::collections::HashSet;

use crate::transpiler::{ParseErr, TokenLoc};
use crate::Token;

pub struct Lexer<'a> {
    content: &'a str,
    token_idx: usize,
    idx: usize,
    live_pos: (usize, usize),
    last_token_pos: (usize, usize),
    specials: HashSet<&'static str>,
    current_token: Option<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        Lexer {
            content,
            token_idx: 0,
            idx: 0,
            live_pos: (0, 0),
            last_token_pos: (0, 0),
            specials: [
                "<", ">", "=", "==", "!=", "<=>", "<=", ">=", "<<", ">>", "<<=", ">>=", "&", "|", "^", "&&", "||", "^^", "&=", "|=", "^=",
                "&&=", "||=", "^^=", "!", "-", "+", "/", "*", "&", "**", "//", "+=", "-=", "*=", "/=", "%=", "**=", "//=", "$", "\"", "\'",
                "#", ",", ".", "..", ":", "::", "->", "(", ")", "[", "]", "{", "}", "?",
            ]
            .into_iter()
            .collect(),
            current_token: None,
        }
    }

    pub fn peek_char(&mut self) -> Option<char> {
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
                            .fold(0, |acc, (val, (s, b))| acc | (((*val & b) as u32) << s)),
                    )
                };

                Some(ret)
            }
            None => None,
        }
    }

    fn consume_char(&mut self, c: char) {
        self.idx += c.len_utf8();

        if c == '\n' {
            self.live_pos.0 = 0;
            self.live_pos.1 += 1;
        } else {
            self.live_pos.0 += 1;
        }
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

    fn next(&mut self) -> Option<(Token<'a>, TokenLoc)> {
        self.last_token_pos = self.live_pos;

        while self.predicate(char::is_whitespace) {}

        self.token_idx = self.idx;

        let start = self.idx;

        if let Some(next) = self.peek_char() {
            self.consume_char(next);

            if next.is_alphanumeric() {
                while self.predicate(char::is_alphanumeric) {}

                Some((
                    Token::Textual(unsafe { self.content.get_unchecked(start..self.idx) }),
                    TokenLoc(self.token_idx, self.idx),
                ))
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

                Some((
                    Token::Special(unsafe { self.content.get_unchecked(start..self.idx) }),
                    TokenLoc(self.token_idx, self.idx),
                ))
            }
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<Token<'a>> {
        let idx = self.idx;
        let token_idx = self.token_idx;
        let live_pos = self.live_pos;
        let last_pos = self.last_token_pos;

        let ret = self.next();

        self.idx = idx;
        self.token_idx = token_idx;
        self.live_pos = live_pos;
        self.last_token_pos = last_pos;

        ret.0
    }

    pub fn cur_next(&mut self) -> Option<(Token<'a>, TokenLoc)> {
        let ret = self.current_token;

        self.current_token = self.next().0;

        ret.map(|val| (val, TokenLoc(self.token_idx, self.idx)))
    }

    pub fn next_cur(&mut self) -> Option<Token<'a>> {
        self.current_token = self.next().0;

        self.current_token
    }

    pub fn cur(&self) -> Option<Token<'a>> {
        self.current_token
    }

    pub unsafe fn get_from_to(&self, from: TokenLoc, to: TokenLoc) -> &'a str {
        unsafe { self.content.get_unchecked(from.0..to.1) }
    }

    pub fn token_idx(&self) -> usize {
        self.token_idx
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn pos(&self) -> (usize, usize) {
        self.last_token_pos
    }
}
