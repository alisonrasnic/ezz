
use crate::trie::TrieNode;
use crate::tree_generator::TreeGenerator;
use crate::compiler_context::CompilerContext;

use myl_tree::{Tree, TreeNode};

use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub struct Parser {
    trie_1: TrieNode,
    trie_2: TrieNode,
    vars:   Vec<String>,
    funcs:  Vec<String>,
}

impl Parser {
    pub fn new() -> Parser {
        let mut trie_1 = TrieNode::new();
        trie_1.insert_route(vec![1, 4, 10]);

        trie_1.insert_route(vec![10, 10, 10]);
        
        trie_1.insert_route(vec![5, 2, 12]);

        Parser { trie_1: trie_1, trie_2: TrieNode::new(), vars: vec![], funcs: vec![String::from("let"), String::from("put")] }
    }

    // lex is necessary to convert from string into a token
    //  as opposed to the parse which changes tokens into simpler tokens
    

    pub fn parse(&mut self, tokens: Vec<ParserToken>, context: &mut CompilerContext, parse_stack: &mut Vec<ParserToken>) {
        let mut cur_tok_idx: usize = 0;

        while cur_tok_idx < tokens.len() {
            parse_stack.push(tokens[cur_tok_idx].clone());
            self.full_reduce_1(context, parse_stack);
            self.step(&tokens, &mut cur_tok_idx);
        }
    }

    pub fn step(&self, tokens: &Vec<ParserToken>, cur_token: &mut usize) -> Result<&'static str, &'static str> 
    {
        // step simply moves cur_token to our next token
        *cur_token += 1;

        Ok("Success")
    }

    pub fn full_reduce_1(&mut self, context: &mut CompilerContext, parse_stack: &mut Vec<ParserToken>) -> Result<&'static str, &'static str> {

        // full_reduce executes reduce repeatedly on our entire stack until the very last
        // possibility of reducing returns Err

        let mut stack_beg = 0;
        let mut stack_wid = 1;
        
        while stack_wid < parse_stack.len() {

            let mut end_idx = stack_beg+stack_wid;
            if end_idx > parse_stack.len() {
                end_idx = parse_stack.len()-1;
            }

            let tok_id = parse_stack[stack_beg].id;
            let result = self.reduce(&parse_stack[stack_beg..end_idx], &context.files[tok_id]);             
            let literal = Parser::string_from_p_slice(&parse_stack[stack_beg..end_idx], &context.files[tok_id]);

            if let Ok(p_type) = result {
                println!("Reduction!"); 

                use std::ptr::NonNull;

                let tok_start = parse_stack[stack_beg].start;
                let tok_end   = parse_stack[end_idx].end;
                let tok_line  = parse_stack[stack_beg].line;

                for i in (stack_beg..end_idx).rev() {
                    let token = parse_stack.remove(i);
                    let mut tree_new_node = context.gen.take_mut(token);
                    context.tree.set_head(tree_new_node);
                    println!("Hi");
                    context.tree.print_vlr();
                    println!("Bye");
                    if end_idx > 0 { 
                        end_idx-=1;
                    }
                }

                // for creation of parser tokens, we are going to require a registry of
                // parsertokens to keep track of their IDs
                parse_stack.insert(stack_beg, ParserToken::new(p_type.clone(), tok_id, tok_start, tok_end, tok_line));
                let mut tree_reduction_node = context.gen.take_mut(ParserToken::new(p_type, tok_id, tok_start, tok_end, tok_line));

                context.tree.set_head(tree_reduction_node);
                std::mem::forget(tree_reduction_node);
                context.tree.print_vlr();

                stack_beg = 0;
                stack_wid = 0;

            } else {
               // println!("Error: {:?}", result);
                stack_beg += 1;
                //println!("beg: {}, wid: {}, parse_stack_len: {}", stack_beg, stack_wid, parse_stack.len());

                if stack_beg >= parse_stack.len() {
                    stack_beg = 0;
                    stack_wid += 1;
                    println!("beg: {}, wid: {}, parse_stack_len: {}", stack_beg, stack_wid, parse_stack.len());
                }
            }
        } 

        Ok("Success")
    }

    pub fn full_reduce_2(&mut self, parse_stack: &mut Vec<ParserToken>, tree: &mut Tree<ParserToken>) -> Result<&'static str, &'static str> {

        // pass 2
        //   here we are processing fnheaders to find their name (2nd token in the tree)
        //   add to the list
        //
        //   also adding vars definitions to our list
        //     this includes func parameters
        //     scope analysis can be done through the tree formed
        //

        Ok("Success")
    }

    fn reduce(&mut self, slice: &[ParserToken], path: &PathBuf) -> Result<ParserTokenType, &'static str> {
        // reduce does a single reduce of a stack of tokens
        let mut types: Vec<u8> = vec![];
        for t in slice {
            println!("-- reduce: found type->{:?} | literal: {:?}", t.get_type(), t.get_literal(path));
            types.push(t.get_type() as u8);
        }

        let res = self.get_regex(types);

        if let Some(v) = res {
            return Ok(v);
        } else {
            return Err("Failed to reduce");
        }
    }

    fn string_from_p_slice(slice: &[ParserToken], path: &PathBuf) -> String {
        let mut ret_string = String::new();
        for t in slice {
            ret_string.push_str(t.get_literal(path));
            ret_string.push(' ');
        }

        ret_string
    }

    // borken!!
    fn get_regex(&mut self, vals: Vec<u8>) -> Option<ParserTokenType> {
        use std::rc::Rc;
        let r = self.trie_1.get_child_from_route(vals);

        if let Some(ref s) = r {
            if let Ok(trie_node) = Rc::try_unwrap(s.into()) {;
                return Some(ParserTokenType::from_u8(trie_node.borrow().get_leaf().unwrap()));
            } else {
                panic!("Rc unwrap failure: {:?}", r.unwrap().borrow() );
            }
        } else {
            //println!("Oof");
        }

        None
    }

    
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ParserToken {
    id:         usize,
    parse_type: ParserTokenType,
    start:      usize,
    end:        usize,
    line:       usize,
}

impl ParserToken {

    pub fn new(parse_type: ParserTokenType, id: usize, start: usize, end: usize, line: usize) -> Self {
        ParserToken { parse_type: parse_type, id: id, start: start, end: end, line: line}
    }

    pub fn get_type(&self) -> ParserTokenType {
        self.parse_type.clone()
    }
    
    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_end(&self) -> usize {
        self.end
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_literal(&self, path_buf: &PathBuf) -> &'static str {
        
        let msg = std::fs::read_to_string(path_buf);
        if let Err(e) = msg {

            panic!("{:?}", e);

        }

        msg.unwrap()[self.start..self.end].to_owned().leak()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParserTokenType {
    Value=2,
    Expr=3,
    Func=6,
    Id=4,
    Op=5,
    Type=1,
    Delim=7,
    Comma=8,
    FuncList=9,
    FuncHeader=10,
    Declare=11,
    Assignment=12,
}

impl ParserTokenType {
    pub fn from_u8(num: u8) -> ParserTokenType {
        match num {
            1 => ParserTokenType::Type,
            2 => ParserTokenType::Value,
            3 => ParserTokenType::Expr,
            4 => ParserTokenType::Id,
            5 => ParserTokenType::Op,
            6 => ParserTokenType::Func,
            7 => ParserTokenType::Delim,
            8 => ParserTokenType::Comma,
            9 => ParserTokenType::FuncList,
            10 => ParserTokenType::FuncHeader,
            11 => ParserTokenType::Declare,
            12 => ParserTokenType::Assignment,
            _ => ParserTokenType::Id,
        }
    }
}
