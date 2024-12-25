use std::io;
use std::io::prelude::*;
use std::fs::File;

mod parser;

use parser::Parser;

fn main() -> io::Result<()> {
    /*
     * 
     * so we could have stuff like
     * let identifer = "abcdefghijklmnopqrstuvwxyz0123456789_";
     *
     * then we check if it matches the contents, so it's a regex problem essentially
     *
     * let digit = "0123456789";
     * let op = "=+-*//*^&|";
        and shit
    */

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

    for x in &stack {
        if *x != String::new() {
            println!("{}", *x);
        }
    }

    let mut parser = Parser::new();

    parser.process_tokens(&stack);

    println!("LexTokens: {:?}", &parser.tokens);

    Ok(())
}
