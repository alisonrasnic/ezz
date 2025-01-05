use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

mod parser;

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
    for x in buffer {
        if x != b' ' && x != b'\n' && x != b'\t' && x != b'\r' {
            stack[idx].push(x as char);
        } else if x != b'\'' && x != b'\"' && !string_start {
            idx += 1;
            stack.push(String::new());
            string_start = false;
        } else {
            if string_start {
                stack[idx].push(x as char);
            } else {
                idx += 1;
                stack.push(String::from(x as char));
                string_start = true;
            }
        }
    }

    let mut parser = Parser::new();

    let result = parser.lex(&stack);

    println!("LexTokens: {:?}", result);

    Ok(())
}
