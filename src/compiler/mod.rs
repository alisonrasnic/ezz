
use crate::parser::{ParserToken, ParserTokenType};
use myl_tree::{Cursor, TreeNode, Tree};

struct Compiler {
    text: String,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { text: String::new() } 
    }

    pub fn compile(&mut self, tree: Tree<ParserToken>) {

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

        let cur_node: Option<Box<TreeNode<ParserToken>>> = tree.get_head();
       
        while let Some(b) = cur_node {
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

                if let Some(body) = (*b).get_right().expect("Expected FnBody node to the right of FuncHeader node") {
                    let return_node = (*body).get_right();
                    let start_code_node = (*body).get_left();

                    self.compile_body(&mut b);
                }
           }

            cur_node = (*b).get_left();
        }
    }

    // this function assumes cur_node is currently pointing at the start of the FnBody's code
    pub fn compile_body(&mut self, cur_node: &mut Box<TreeNode<ParserToken>>) {
        while cur_node.get_left() != None {
            // we need to read our current node, and then go left
            if *cur_node.get_type() == ParserTokenType::Api {
                let string = *(cur_node.get_left().expect("Expected Identifier Node left of Api Node")).get_elem();

                // our node is an api node
                // our next node is NOT string
                // it's actually identifier which we use to identify which api function to use
                // then the next node is string which we pass into the assembly text

                let id = cur_node.get_left().expect("Unreachable").unwrap().get_elem();

                if id.get_type() == ParserTokenType::Identifier {
                    let next_node = cur_node.get_left().unwrap().unwrap().get_left().unwrap();
                    let literal = &context.files[id.get_id()][id.get_start()..id.get_end()]; 
                    let fasm_literal = &context.files[id.get_id()][next_node.get_start()..next_node.get_end()];

                    if literal == "fasm" {
                        self.text.push(fasm_literal);
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
