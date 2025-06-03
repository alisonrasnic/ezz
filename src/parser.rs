
use crate::trie::TrieNode;
use myl_tree::{Tree, TreeNode};

#[derive(Debug, PartialEq, Clone)]
pub struct Parser {
    trie: TrieNode,
}

impl Parser {
    pub fn new() -> Parser {
        let mut trie = TrieNode::new();
        trie.insert_route(vec![2, 5, 2, 3]);
        trie.insert_route(vec![1, 4, 1, 4, 10]);
        // the 8 is going to loop around to the 2nd 1
        trie.insert_route(vec![1, 4, 1, 4, 8, 10]);
        let node_1 = trie.get_child_from_route(vec![1, 4, 1, 4, 8]).unwrap();
        let node_2 = trie.get_child_from_route(vec![1, 4, 1]).unwrap();
        node_1.borrow_mut().insert_child(1, node_2);
        trie.insert_route(vec![4, 4, 5, 2, 3]);
        trie.insert_route(vec![4, 2, 3]);
        //trie.insert_route(vec![3, 7, 3]);
        trie.insert_route(vec![3, 3]);
        trie.insert_route(vec![3, 5, 3, 3]);
        //trie.insert_route(vec![10, 7, 3]);
        trie.insert_route(vec![10, 3]);
        trie.insert_route(vec![10, 4, 3, 2, 6]);
        let node_3 = trie.get_child_from_route(vec![10, 3]).unwrap();
        node_3.clone().borrow_mut().insert_child(3, node_3);
        //trie.insert_route(vec![10, 7, 2, 7, 6]);
        //trie.insert_route(vec![10, 7, 3, 2, 7, 6]);
        trie.insert_route(vec![10, 2, 6]);
        trie.insert_route(vec![10, 3, 2, 6]);
        trie.insert_route(vec![6, 6, 9]);
        trie.insert_route(vec![6, 6, 9]);
        trie.insert_route(vec![9, 6, 9]);
        trie.insert_route(vec![4, 3, 3]);
        trie.insert_route(vec![4, 4, 3]);
        trie.insert_route(vec![3, 5, 2, 3]);

        Parser { trie: trie }
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

    pub fn parse(&mut self, tokens: Vec<ParserToken>, parse_stack: &mut Vec<ParserToken>) -> Tree<ParserToken> {
        let mut tree = Tree::<ParserToken>::new();

        let mut cur_tok_idx: usize = 0;

        while cur_tok_idx < tokens.len() {
            parse_stack.push(tokens[cur_tok_idx].clone());
            self.full_reduce(parse_stack);
            self.step(&tokens, &mut cur_tok_idx);
        }

        tree.set_head(&mut TreeNode::new(parse_stack[0].clone()));

        tree
    }

    pub fn step(&self, tokens: &Vec<ParserToken>, cur_token: &mut usize) -> Result<&'static str, &'static str> 
    {
        // step simply moves cur_token to our next token
        *cur_token += 1;

        Ok("Success")
    }

    pub fn full_reduce(&mut self, parse_stack: &mut Vec<ParserToken>) -> Result<&'static str, &'static str> {

        // full_reduce executes reduce repeatedly on our entire stack until the very last
        // possibility of reducing returns Err

        let mut stack_beg = 0;
        let mut stack_wid = 1;
        
        while stack_wid <= parse_stack.len() {

            let mut end_idx = stack_beg+stack_wid;
            if end_idx > parse_stack.len() {
                end_idx = parse_stack.len()-1;
            }

            let result = self.reduce(&parse_stack[stack_beg..end_idx]);             

            if let Ok(p_type) = result {
                println!("Reduction!"); 
                for i in (stack_beg..stack_wid+1).rev() {
                    parse_stack.remove(i);
                }

                parse_stack.insert(stack_beg, ParserToken::new(p_type, Parser::string_from_p_slice(&parse_stack[stack_beg..stack_beg+stack_wid])));

                stack_beg = 0;
                stack_wid = 0;

            } else {
                println!("Error: {:?}", result);
                stack_beg += 1;

                if stack_beg >= parse_stack.len() {
                    stack_beg = 0;
                    stack_wid += 1;
                }
            }
        } 

        Ok("Success")
    }

    fn reduce(&mut self, slice: &[ParserToken]) -> Result<ParserTokenType, &'static str> {
        // reduce does a single reduce of a stack of tokens
        let mut types: Vec<u8> = vec![];
        for t in slice {
            println!("-- reduce: found type->{:?} | literal: {:?}", t.get_type() as u8, t.get_literal());
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
        }

        ret_string
    }

    // borken!!
    fn get_regex(&mut self, vals: Vec<u8>) -> Option<ParserTokenType> {
        use std::rc::Rc;
        let r = self.trie.get_child_from_route(vals);

        if let Some(s) = r {
            if let Ok(trie_node) = Rc::try_unwrap(s) {;
                return Some(ParserTokenType::from_u8(trie_node.borrow().get_leaf().unwrap()));
            }
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
            _ => ParserTokenType::Id,
        }
    }
}
