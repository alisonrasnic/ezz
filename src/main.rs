use std::io;
use std::io::prelude::*;
use std::fs::File;

mod parser;

use parser::Parser;

fn main() -> io::Result<()> {
    let mut f = File::open("mn.ezz")?;
    let mut buffer = [0; 1000];
    let mut stack: Vec<String> = vec![String::new()];

    let _ = f.read(&mut buffer)?;

    let mut idx = 0; 
    for x in buffer {
        if x != b' ' && x != b'\n' && x != b'\t' && x != b'\r' {
            stack[idx].push(x as char);
        } else {
            idx += 1;
            stack.push(String::new());
        }
    }

    let mut parser = Parser::new();

    parser.lex_tokens(&stack);

    println!("LexTokens: {:?}", &parser.lex_tokens);

    Ok(())
}
