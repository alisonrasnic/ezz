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
        let mut col = 0;
        let mut lexes: Vec<ParserToken> = vec![];
        
        let mut start: usize = 0;

        let mut id = self.register(path.clone(), context).expect("Could not register path");

        for i in 1..(text_len+1) {
            let cur_char = text_chars.next().unwrap();
            if cur_char == '\0' {
                println!("EOF encountered");
                break;
            } else if cur_char.is_whitespace() {
            }

            if Self::str_matches_lexes(&text[start..i], context) {

                if !Self::str_matches_lexes(&text[start..i+1], context) {

                    // We know we reached the end of our current token
                    let res = Self::str_is_lex(&text[start..i], context);

                    if let Some(s) = res {

                        lexes.push(ParserToken::new(s, start, i, line_count, id));
                        println!("Token added type: {:?}, name: {}", s, &text[start..i]);
                        start = i;

                    } else {
                        lexes.push(ParserToken::new(ParserTokenType::Id, start, i, line_count, id));
                        println!("Identifier added, name: {}", &text[start..i]);
                        start = i;
                    }
                }

            } else {
                if cur_char.is_whitespace() {
                    println!("Whitespace encountered with symbol: {:?}", &text[start..i]);
                    start = i;    
                } else {
                    panic!("Unknown symbol: {:?}", &text[start..i]);
                }
            }

            col += 1;
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
                return Ok(context.files.len()-1);
            }
        } else {
            return Err(format!("path: {:?} does not exist!", path).leak());
        }
    }

    fn str_to_lex(s: &str, c: &mut CompilerContext) -> Result<ParserTokenType, &'static str> {

        Err("TODO")

    }

    fn str_matches_lexes(s: &str, c: &mut CompilerContext) -> bool {

        // String
        if s.starts_with('"') {
            if s.chars().filter(|c| *c == '"').count() == 2 && s.ends_with('"') {
                return true;
            } else if s.chars().filter(|c| *c == '"').count() == 1 {
                return true;
            }
        }

        // Char
        if s.starts_with('\'') {
            return true;
        }

        // Negative numbers
        if s.starts_with('-') {

            let new_s = &s[1..];

            // Integers
            if new_s.chars().all(|ch: char| ch.is_digit(10)) {
                return true;
            }

            // Floats
            if let Some(t) = new_s.split_once('.') {
                if t.0.starts_with(|ch: char| ch.is_digit(10)) || t.0 == "" {
                    if t.1.chars().all(|ch: char| ch.is_digit(10)) {
                        return true;
                    }
                }

            }
        } else {

            // Integers
            if s.chars().all(|ch: char| ch.is_digit(10)) {
                return true;
            }

            // Floats
            if let Some(t) = s.split_once('.') {
                if t.0.starts_with(|ch: char| ch.is_digit(10)) || t.0 == "" {
                    if t.1.chars().all(|ch: char| ch.is_digit(10)) {
                        return true;
                    }
                }

            }
        }

        // Booleans
        if "true".starts_with(s) || "false".starts_with(s) {
            return true;
        }

        // Types
        for t in &c.types {
            if t.starts_with(s) {
                return true;
            }
        }

        // Keyword/funcs
        for f in &c.funcs {
            if f.starts_with(s) {
                return true;
            }
        }

        // Identifiers
        if s.chars().all(char::is_alphanumeric) && !s.chars().nth(0).expect("no char").is_digit(10) {
            return true;
        }

        false
    }

    fn str_is_lex(s: &str, c: &mut CompilerContext) -> Option<ParserTokenType> {

        // String
        if s.starts_with('"') && s.ends_with('"') {
            return Some(ParserTokenType::Str);
        }

        // Char
        if s.starts_with('\'') && s.ends_with('\'') {
            return Some(ParserTokenType::Ch);
        }

        {
            // Negative
            if s.starts_with('-') {
                let s = &s[1..];
            }
            // Integers
            if s.chars().all(|ch: char| ch.is_digit(10)) {
                return Some(ParserTokenType::Num);
            }

            // Floats
            if let Some(t) = s.split_once('.') {

                if t.0.starts_with(|ch: char| ch.is_digit(10)) || t.0 == "" {
                    if t.1.chars().all(|ch: char| ch.is_digit(10)) {
                        return Some(ParserTokenType::Float);
                    }
                }
            }
        }

        // Types
        for t in &c.types {
            if *t == s {
                return Some(ParserTokenType::Type);
            }
        }

        for f in &c.funcs {
            if *f == s {
                return Some(ParserTokenType::Func);
            }
        }

        // Booleans
        if s == "true" || s == "false" {
            return Some(ParserTokenType::Bool);
        }

        None
    }
}
