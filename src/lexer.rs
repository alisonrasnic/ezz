use std::collections::HashMap;
use std::path::PathBuf;

use crate::parser::{ParserTokenType, ParserToken};

struct Lexer {
    files: Vec<PathBuf>,
}

impl Lexer {
    pub fn new() -> Self {

        Lexer { files: vec![] }
    }

    pub fn lex(&mut self, text: String, path: PathBuf) -> Vec<ParserToken> {

        let mut text_chars = text.chars();
        let text_len   = text_chars.clone().count();

        let mut line_count = 0;
        let mut cur_line_char = 0;
        let mut lexes: Vec<ParserToken> = vec![];
        
        let mut start: usize = 0;

        for i in 0..(text_len) {
            let cur_char = text_chars.next().unwrap();
            if cur_char == '\n' {
                line_count += 1;
                cur_line_char = 0;
            } else if cur_char == '\t' {
                i += 1;
                cur_line_char += 1;
                continue;
            }

            let res = Self::str_to_lex((&text[start..i]));

            if let Ok(s) = res {
                let p_token = ParserToken::new(s, self.files.len(), start, i, line_count);
                start = i;
                lexes.push(p_token);
            }

            cur_line_char += 1;
        }

        self.files.push(path);
        lexes
    }

    pub fn register(&mut self, path: PathBuf) -> Result<usize, &'static str> {

        if path.exists() {

            if self.files.contains(&path) {
                let idx = self.files.iter().position(|x| *x == path).unwrap();
                return Ok(idx);
            } else {
                self.files.push(path);
                return Ok(self.files.len());
            }
        } else {

            return Err(format!("path: {:?} does not exist!", path).leak());
        }

    }

    fn str_to_lex(s: &'static str) -> Result<ParserTokenType, &'static str> {
        let typ = ParserTokenType::Id;
        let mut is_dig = true;
        for ch in s.chars() {
            if Self::match_ch_to_rx(|x| !x.is_digit(10), ch) {
                is_dig = false;
            }

            if is_dig {
                return Ok(ParserTokenType::Value);
            }
        }

        if Self::match_str_to_rx(|st| st.starts_with("\"") && st.ends_with("\""), &s.to_owned()) {
            return Ok(ParserTokenType::Value);
        } else if Self::match_str_to_rx(|st| *st == String::from("$") || *st == String::from("i32") || *st == String::from("u32") || *st == String::from("string") || *st == String::from("bool"), &s.to_owned()) {
            return Ok(ParserTokenType::Type);
        } else if Self::match_str_to_rx(|st| *st == String::from("{") || *st == String::from("}") || *st == String::from(";"), &s.to_owned()) {
            return Ok(ParserTokenType::Delim);
        } else if Self::match_str_to_rx(|st| *st == String::from("=") || *st == String::from("+") || *st == String::from("-") || *st == String::from("*") || *st == String::from("/"), &s.to_owned()) {
            return Ok(ParserTokenType::Op);
        } else {
            return Ok(ParserTokenType::Id);
        }
    }

    fn match_str_to_rx<F>(f: F, st: &String) -> bool where
        F: Fn(&String) -> bool {
        
        f(st)
    }

    fn match_ch_to_rx<F>(f: F, st: char) -> bool where
        F: Fn(char) -> bool {
        
        f(st)
    }
}
