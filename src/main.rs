use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

mod parser;
mod trie;

use parser::Parser;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(args[1].as_str())?;
    let mut buffer = [0; 1000];
    let mut stack: Vec<String> = vec![String::new()];

    let _ = f.read(&mut buffer)?;

    /*
     *
     * so for this section we need to account for certain things
     * like strings can contain whitespaces
     *
     * what else...
     *
     * this is where comments would be cutout
     * preprocessing import statements
     *
     */
    let mut idx = 0; 
    let mut string_start = false;
    let mut skip_until = 0;
    for i in 0..1000 {
        let x = buffer[i];
        if x != b'\0' && skip_until == 0 {
            if i+1 < buffer.len() && x == b'/' && buffer[i+1] == b'/' {
                let mut new_i = i;
                while buffer[new_i] != b'\n' {
                    new_i += 1;
                    skip_until += 1;
                }
                continue;
            } else if x != b'$' && x != b' ' && x != b'\n' && x != b'\t' && x != b'\r' && x != b'\'' && x != b'\"' {
                stack[idx].push(x as char);
            } else if x != b'\'' && x != b'\"' && !string_start && x != b'$' {
                idx += 1;
                stack.push(String::new());
                string_start = false;
            } else {
                if string_start {
                    stack[idx].push(x as char);
                    if x == b'\'' || x == b'\"' {
                        idx += 1;
                        stack.push(String::new());
                        string_start = false;
                    }
                } else {
                    idx += 1;
                    stack.push(String::from(x as char));
                    if x == b'\'' || x == b'\"' {
                        string_start = true;
                    } else {
                        idx += 1;
                        stack.push(String::new());
                    }
                }
            }
        } else if skip_until > 0 {
            skip_until -= 1;
        }
    }

    let mut parser = Parser::new();

    use crate::parser::ParserToken;
    let mut result: Vec<ParserToken> = vec![];
    result = parser.lex(&stack);
    let mut parse_stack: Vec<ParserToken> = vec![];
    println!("\n\nBeginning parsing...\n\n");
    let parse_res = parser.parse(result, &mut parse_stack);

    println!("\n\n\n\n\n\n");

    for x in parse_stack {
        println!("{:?}, ", x);
    }

    use colored::Colorize;
    if parse_res {
        println!("\n{}", "Parsing successful!".green());
    } else {
        println!("\n{}", "Parsing failed with errors...".red());
    }

    Ok(())
}
