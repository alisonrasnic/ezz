use std::collections::HashMap;
use std::path::PathBuf;

use crate::parser::{ParserTokenType, ParserToken};
use crate::compiler_context::CompilerContext;

pub struct Lexer; 

impl Lexer {
    pub fn new() -> Self {

        Lexer { }
    }

    pub fn lex(&mut self, text: String, path: PathBuf, context: &mut CompilerContext) -> Vec<ParserToken> {

        let mut text_chars = text.chars();
        let text_len   = text_chars.clone().count();

        let mut line_count = 0;
        let mut cur_line_char = 0;
        let mut lexes: Vec<ParserToken> = vec![];
        
        let mut start: usize = 0;

        for i in 1..(text_len+1) {
            let cur_char = text_chars.next().unwrap();
            if cur_char == '\0' {
                println!("EOF encountered");
                break;
            }

            if cur_char == '\n' {
                line_count += 1;
                cur_line_char = 0;
                start = i;
                continue;
            } else if cur_char == '\r' || cur_char == '\t' || cur_char == ' ' {
                start = i;
                cur_line_char += 1;
                continue;
            }

            let res = Self::str_to_lex((&text[start..i]), context);

            if let Ok(s) = res {
                let p_token = ParserToken::new(s, context.files.len(), start, i, line_count);
                start = i;
                lexes.push(p_token);
            } else if let Err(msg) = res {
                println!("{}", msg);
            }

            cur_line_char += 1;
        }

        context.files.push(path);
        lexes
    }

    pub fn register(&mut self, path: PathBuf, context: &mut CompilerContext) -> Result<usize, &'static str> {

        if path.exists() {

            if context.files.contains(&path) {
                let idx = context.files.iter().position(|x| *x == path).unwrap();
                return Ok(idx);
            } else {
                context.files.push(path);
                return Ok(context.files.len());
            }
        } else {

            return Err(format!("path: {:?} does not exist!", path).leak());
        }

    }

    fn str_to_lex(s: &str, c: &mut CompilerContext) -> Result<ParserTokenType, &'static str> {

        if !s.is_ascii() {
            return Err("Non-ascii string");
        }

        let mut hits = 0;
        let mut typ  = ParserTokenType::Id;

        let mut is_dig = true;
        for ch in s.chars() {
            if Self::match_ch_to_rx(|x| !x.is_digit(10), ch) {
                is_dig = false;
            }

            if is_dig {
                typ = ParserTokenType::Value;
                hits += 1;
            }
        }

        if Self::match_str_to_rx(|st| st.starts_with("\"") && st.ends_with("\""), &s.to_owned()) {
            typ = ParserTokenType::Value;
            hits += 1;
        } else if Self::match_str_to_rx(|st| c.types.contains(&st.as_str()), &s.to_owned()) {
            typ = ParserTokenType::Type;
            hits += 1;
        } else if Self::match_str_to_rx(|st| *st == String::from("{") || *st == String::from("}") || *st == String::from(";"), &s.to_owned()) {
            typ = ParserTokenType::Delim;
            hits += 1;
        } else if Self::match_str_to_rx(|st| c.funcs.contains(&st.as_str()), &s.to_owned()) {
            typ = ParserTokenType::Func;
            hits += 1;
        } else if Self::match_str_to_rx(|st| st.chars().all(char::is_alphanumeric) && !st.chars().nth(0).expect("").is_digit(10), &s.to_owned()) {
            typ = ParserTokenType::Id;
            hits += 1;
        }

        if hits == 1 {
            return Ok(typ);
        } else {
            if hits == 0 {
                return Err(format!("No matches for s: {}", s).leak());
            } else {
                return Err("Inconclusive lex");
            }
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
