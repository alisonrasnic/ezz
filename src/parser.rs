
use crate::trie::TrieNode;
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct Parser {
    trie: TrieNode,
}

impl Parser {
    pub fn new() -> Parser {
        let mut trie = TrieNode::new();
        trie.insert_route(vec![2, 5, 2, 3]);
        trie.insert_route(vec![1, 4, 1, 4, 6]);
        // the 8 is going to loop around to the 2nd 1
        trie.insert_route(vec![1, 4, 1, 4, 8, 6]);
        let mut node_1 = trie.get_child_from_route(vec![1, 4, 1, 4, 8]).unwrap();
        let node_2 = trie.get_child_from_route(vec![1, 4, 1]).unwrap();
        node_1.borrow_mut().insert_child(1, node_2);
        trie.insert_route(vec![4, 4, 5, 2, 3]);
        trie.insert_route(vec![4, 2, 2]);
        trie.insert_route(vec![6, 7, 6]);
        trie.insert_route(vec![6, 4, 3, 2, 6]);
        let node_3 = trie.get_child_from_route(vec![6, 7, 6]).unwrap();
        node_3.clone().borrow_mut().insert_child(6, node_3);
        trie.insert_route(vec![6, 7, 2, 7, 6]);

        Parser { trie: trie }
    }

    // lex is necessary to convert from string into a token
    //  as opposed to the parse which changes tokens into simpler tokens
    pub fn lex(&self, tokens: &Vec<String>) -> Vec<ParserToken> {
        let mut lexes: Vec<ParserToken> = vec![];
        
        for s in tokens {
            if *s != String::new() {
                let st_s = s.clone().leak();
                lexes.push(ParserToken { parse_type: Self::str_to_lex(st_s).expect("Failed to lex token"), literal: st_s });
            }
        }

        lexes
    }

    pub fn parse<'a>(&self, tokens: Vec<ParserToken<'a>>, parse_stack: &mut Vec<ParserToken<'a>>) -> bool {
        let mut cur_token: Option<ParserToken> = None;
        let mut tokens_iter = tokens.iter();
        cur_token = tokens_iter.next().cloned();

        while cur_token.is_some() {
            let t = cur_token.unwrap();
            parse_stack.push(t);
            let mut res = self.reduce(parse_stack);
            while res.is_ok() {
                res = self.reduce(parse_stack);
            }
            match res {
                Ok(r) => {},
                Err(t) => {
                    if t != "eof" {
                        panic!("Parsing error: {}", res.unwrap());
                    }
                },
            }
            cur_token = tokens_iter.next().cloned();
        }

        parse_stack[0].parse_type == ParserTokenType::Func
    }

    pub fn reduce<'a>(&self, mut parse_stack: &mut Vec<ParserToken<'a>>) -> Result<&'static str, &'static str> {
        // This is where we use the trie to follow our established rules
        //
        
        let mut cur_trie_node = self.trie.clone();
        let mut local_stack: Vec<u8> = vec![];
        for x in &mut *parse_stack {
            local_stack.push(x.parse_type.clone() as u8);
        }

        let mut res = cur_trie_node.get_child_from_route(local_stack);

        match res {
            Some(s) => {
                let leaf = s.borrow_mut().get_leaf().unwrap();

                *parse_stack = vec![ParserToken { parse_type: from_u8(leaf), literal: ""}];
                
                Ok("success")
            },
            None    => {Err("eof")}
        }
    }

    fn str_to_lex(s: &'static str) -> Option<ParserTokenType> {
        let mut typ = ParserTokenType::Id;
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
pub struct ParserToken<'a> {
    parse_type: ParserTokenType,
    literal: &'a str,
}

// what kinds of things do we really need in parser tokens?
// value op value reduces to value
//
//   ex:  
//          i32 mn $args {
//              put "Hello, World!";
//              let x = 5 + 5;
//              0
//          }
//
//  this whole thing needs to reduce to an acceptable token
//
//  this is why the C expr token exists
//
//  so the whole thing needs to eval to expr
//
//  lets reduce the whole example so far
//
//  Type Reserved Arr Type Reserved Delimiter Id String Id Id Op Value Op Value Value Delimiter
//
//  Function Reserved Delimiter Function Expr Value Delimiter
//  Function Reserved Expr
//  Function
//
//  I think Func can also work as a final value, we just call the main function if it exists
//
//  * = 0+
//  Grammar Rules:
//      Type Id (Type Id Comma)* => Func
//
//      Id Value => Func
//
//      Id Id Op Value => Expr
//
//      Value Op Value => Expr
//
//      Func Id Expr Value => Func
//
//      String => Value
//
//  Let's interpret the ex using these rules and LR(1) parsing
//
//      Shift
//
//      "i32" "mn"
//      type   mn
//      type   id  $args
//      type   id  type id
//      
//      func delimiter
//      func delimiter put
//      func delimiter id "Hello, world!"
//      func delimiter id value
//      func delimiter func
//      func delimiter func let
//      func delimiter func id  x
//      func delimiter func id  id
//      func delimiter func id  id =
//      func delimiter func id  id op
//      func delimiter func id  id op 5
//      func delimiter func id  id op value
//      func delimiter func expr
//      func delimiter func func 0
//      func delimiter func func value
//      func delimiter func func value delimiter
//      func delimiter func func value delimiter
//
//      we halt there on current rules
//
//      we add one more rule:
//          func delimiter func* value delimiter => func
//
//      now we get our last reduce:
//
//      func
//
#[derive(Debug, PartialEq, Clone)]
pub enum ParserTokenType {
    Value=2,
    Expr=3,
    Func=6,
    Id=4,
    Op=5,
    Type=1,
    Delim=7,
    Comma=8
}

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
        _ => ParserTokenType::Id,
    }
}
