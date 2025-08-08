
use crate::parser::{ParserToken, ParserTokenType};
use crate::CompilerContext;
use myl_tree::{Cursor, TreeNode, Tree};

pub mod Target;
pub mod FASM;

use FASM::*;
use Target::{CompilerTarget};

pub struct Compiler {
    target: CompilerTarget,
    text: String,
    data: Vec<String>,
}

impl Compiler {
    pub fn new(target: CompilerTarget) -> Self {
        let mut text = String::new();
        match target {
            CompilerTarget::FasmWinx86 => {
                text.push_str(FasmWinx86Header);
                text.push_str("\tstart:\n\t\t");
            },
            _ => (),
        };

        Compiler { target: target, text: text, data: vec![] } 
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn compile(&mut self, tree: Tree<ParserToken>, context: &mut CompilerContext) {

        // We are compiling Ezz AST into FASM
        // For example
        //   we get a tree
        //                       FnHeader
        //                   i32 mn      FnBody
        //                          put "Hi"   0      
        //
        //
        //   however, the standard mn function requires core
        //
        //   let's write some core functions first, like put
        //
        //                       FnHeader
        //                   fn put                                                FnBody
        //                str inp  |   api append_fasm "<INSERT FASM HELLO, WORLD HERE>" 0
        //
        // so we need a StringBuilder

        let mut cur_node: Option<Box<TreeNode<ParserToken>>> = tree.get_head();
       
        while let Some(mut b) = cur_node {
            if (*b).get_elem().get_type() == ParserTokenType::FuncHeader {
                // if we read an fn header
                //      we know the left node contains the type
                //      left node of that contains name
                //      continuing left down gives us arguments, if there are any
                //
                //      right node is the body
                //      left side is function calls, right side is returning value/function
                //
                //      for now, let's ignore left side and deal with the body

                if let Some(body) = (*b).get_right() {
                    let return_node = (*body).get_right();
                    let start_code_node = (*body).get_left();

                    self.compile_body(&mut b, context);
                }
           }

            cur_node = (*b).get_left();
        }
    }

    // this function assumes cur_node is currently pointing at the start of the FnBody's code
    pub fn compile_body(&mut self, cur_node: &mut Box<TreeNode<ParserToken>>, context: &mut CompilerContext) {
        while cur_node.get_left() != None {
            // we need to read our current node, and then go left
            if (*cur_node).get_elem().get_type() == ParserTokenType::Api {
                let string = *((*cur_node).get_left().expect("Expected Identifier Node left of Api Node")).get_elem();

                // our node is an api node
                // our next node is NOT string
                // it's actually identifier which we use to identify which api function to use
                // then the next node is string which we pass into the assembly text

                let node = (*cur_node).get_left().expect("Unreachable");
                let id = node.get_elem();

                if id.get_type() == ParserTokenType::Id {
                    let next_node = cur_node.get_left().unwrap().get_left().unwrap();

                    let literal = id.get_literal(&context.files[id.get_id()]); 
                    let fasm_literal = (*next_node).get_elem().get_literal(&context.files[(*next_node).get_elem().get_id()]); 

                    if literal == "append_fasm" {
                        self.text.push_str(&fasm_literal[1..fasm_literal.len()-1]);
                    } else if literal == "fasm_data" {
                        panic!("TODO: Implement fasm data insertion"); 
                    } else {
                        panic!("Unknown Api call {}", literal);
                    }
                } else {
                    panic!("Expected Identifier Node left of Api Node");
                }
            }

            *cur_node = cur_node.get_left().expect("Unreachable");
        }
    }
}
