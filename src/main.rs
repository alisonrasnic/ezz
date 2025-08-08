use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::PathBuf;

/// Compiler says hi to doc readers!!! c: 
mod compiler;
/// Contains scopes, variables, functions, files, and more for compiling
mod compiler_context;
/// Contains definitions for functions, scopes, and more for ``compiler_context``
mod compiler_info;
/// Typing for the language itself as opposed to parsing
mod ezz_type;
/// Transforms string into ``Vec<ParserToken>``
mod lexer;
/// Transforms ``Vec<ParserToken>`` into a parse tree using ``myl_tree``
mod parser;
/// Trie for RegEx parsing of grammar
mod trie;
/// Owns the tree nodes and parser tokens to create an AST
mod tree_generator;
/// Unit and integration testings
mod tests;

use parser::Parser;
use parser::ParserTokenType;
use tree_generator::TreeGenerator;
use compiler_context::CompilerContext;
use lexer::Lexer;

use myl_tree::Tree;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut path = PathBuf::new();
    path.push(&args[1]);
    let mut f = File::open(args[1].as_str())?;
    let mut buffer = [0; 1000];
    let mut stack: Vec<String> = vec![String::new()];

    let _ = f.read(&mut buffer)?;


    let mut context = CompilerContext::ezz_default();
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

    let mut lexer  = Lexer::new();
    let mut parser = Parser::new();

    use crate::parser::ParserToken;
    let mut result: Vec<ParserToken> = vec![];

    result = lexer.lex(std::str::from_utf8(&buffer).unwrap().to_owned(), path, &mut context);

    println!("\n\nLexing result: \n{:?}\n\n", result);

    let mut parse_stack: Vec<ParserToken> = vec![];

    println!("\n\nBeginning parsing...\n\n");
    parser.parse(result, &mut context, &mut parse_stack);

    println!("\n\n\n\n\n\n");

    for x in parse_stack {
        println!("{:?}, ", x);
    }

    for x in &context.funcs {
        println!("Function: {:?}\n", x);
    }
    /*println!("\n");
    for x in &parse_res {
        println!("{:?}", x);
    }
    println!("\n");*/

    println!("\n\n\n{:?}\n\n\n", context.gen);
    println!("\n\n\n");
    context.gen.get_tree().print_vlr();

    use colored::Colorize;
    {
        //context.tree.print_vlr();
        //println!("{:?}", context.tree);
    }

    Ok(())
}
