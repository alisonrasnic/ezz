
use crate::trie::TrieNode;
use crate::tree_generator::TreeGenerator;
use myl_tree::{Tree, TreeNode};

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
    pub fn lex(&self, tokens: &Vec<String>) -> Vec<ParserToken> {
        let mut lexes: Vec<ParserToken> = vec![];
        
        for s in tokens {
            if *s != String::new() {
                lexes.push(ParserToken { parse_type: Self::str_to_lex(s.clone().leak()).expect("Failed to lex token"), literal: s.clone() });
            }
        }

        lexes
    }

    pub fn parse(&mut self, tokens: Vec<ParserToken>, parse_stack: &mut Vec<ParserToken>, tree: &mut Tree<ParserToken>, tree_generator: &mut TreeGenerator) {
        let mut cur_tok_idx: usize = 0;

        while cur_tok_idx < tokens.len() {
            parse_stack.push(tokens[cur_tok_idx].clone());
            self.full_reduce_1(parse_stack, tree, tree_generator);
            self.step(&tokens, &mut cur_tok_idx);
        }
    }

    pub fn step(&self, tokens: &Vec<ParserToken>, cur_token: &mut usize) -> Result<&'static str, &'static str> 
    {
        // step simply moves cur_token to our next token
        *cur_token += 1;

        Ok("Success")
    }

    pub fn full_reduce_1(&mut self, parse_stack: &mut Vec<ParserToken>, tree: &mut Tree<ParserToken>, tree_generator: &mut TreeGenerator) -> Result<&'static str, &'static str> {

        // full_reduce executes reduce repeatedly on our entire stack until the very last
        // possibility of reducing returns Err

        let mut stack_beg = 0;
        let mut stack_wid = 1;
        
        while stack_wid < parse_stack.len() {

            let mut end_idx = stack_beg+stack_wid;
            if end_idx > parse_stack.len() {
                end_idx = parse_stack.len()-1;
            }

            let result = self.reduce(&parse_stack[stack_beg..end_idx]);             
            let literal = Parser::string_from_p_slice(&parse_stack[stack_beg..end_idx]);

            if let Ok(p_type) = result {
                println!("Reduction!"); 

                use std::ptr::NonNull;

                for i in (stack_beg..end_idx).rev() {
                    let mut tree_new_node = tree_generator.take_mut(parse_stack.remove(i));
                    //tree.set_head(tree_new_node);
                    println!("Hi");
                    tree.print_vlr();
                    println!("Bye");
                    if end_idx > 0 { 
                        end_idx-=1;
                    }
                }

                parse_stack.insert(stack_beg, ParserToken::new(p_type.clone(), literal.clone()));
                let mut tree_reduction_node = tree_generator.take_mut(ParserToken::new(p_type, literal));

                tree.set_head(tree_reduction_node);
                std::mem::forget(tree_reduction_node);

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

    fn reduce(&mut self, slice: &[ParserToken]) -> Result<ParserTokenType, &'static str> {
        // reduce does a single reduce of a stack of tokens
        let mut types: Vec<u8> = vec![];
        for t in slice {
            println!("-- reduce: found type->{:?} | literal: {:?}", t.get_type(), t.get_literal());
            types.push(t.get_type() as u8);
        }

        let res = self.get_regex(types);

        if let Some(v) = res {
            return Ok(v);
        } else {
            return Err("Failed to reduce");
        }
    }

    fn string_from_p_slice(slice: &[ParserToken]) -> String {
        let mut ret_string = String::new();
        for t in slice {
            ret_string.push_str(t.get_literal().leak());
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

    fn str_to_lex(s: &'static str) -> Option<ParserTokenType> {
        let typ = ParserTokenType::Id;
        let mut is_dig = true;
        for ch in s.chars() {
            if Self::match_ch_to_rx(|x| !x.is_digit(10), ch) {
                is_dig = false;
            }

            if is_dig {
                return Some(ParserTokenType::Value);
            }
        }

        if Self::match_str_to_rx(|st| st.starts_with("\"") && st.ends_with("\""), &s.to_owned()) {
            return Some(ParserTokenType::Value);
        } else if Self::match_str_to_rx(|st| *st == String::from("$") || *st == String::from("i32") || *st == String::from("u32") || *st == String::from("string") || *st == String::from("bool"), &s.to_owned()) {
            return Some(ParserTokenType::Type);
        } else if Self::match_str_to_rx(|st| *st == String::from("{") || *st == String::from("}") || *st == String::from(";"), &s.to_owned()) {
            return Some(ParserTokenType::Delim);
        } else if Self::match_str_to_rx(|st| *st == String::from("=") || *st == String::from("+") || *st == String::from("-") || *st == String::from("*") || *st == String::from("/"), &s.to_owned()) {
            return Some(ParserTokenType::Op);
        } else {
            return Some(ParserTokenType::Id);
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

#[derive(Debug, PartialEq, Clone)]
pub struct ParserToken {
    parse_type: ParserTokenType,
    literal: String,
}

impl ParserToken {

    pub fn new(parse_type: ParserTokenType, literal: String) -> Self {
        ParserToken { parse_type: parse_type, literal: literal } 
    }

    pub fn get_type(&self) -> ParserTokenType {
        self.parse_type.clone()
    }

    pub fn get_literal(&self) -> String {

        self.literal.clone()

    }
}

#[derive(Debug, PartialEq, Clone)]
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
