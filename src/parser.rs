
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
        trie.insert_route(vec![1, 4, 1, 4, 10]);
        // the 8 is going to loop around to the 2nd 1
        trie.insert_route(vec![1, 4, 1, 4, 8, 10]);
        let mut node_1 = trie.get_child_from_route(vec![1, 4, 1, 4, 8]).unwrap();
        let node_2 = trie.get_child_from_route(vec![1, 4, 1]).unwrap();
        node_1.borrow_mut().insert_child(1, node_2);
        trie.insert_route(vec![4, 4, 5, 2, 3]);
        trie.insert_route(vec![4, 2, 3]);
        trie.insert_route(vec![3, 7, 3]);
        trie.insert_route(vec![3, 5, 3, 3]);
        trie.insert_route(vec![10, 7, 3]);
        trie.insert_route(vec![10, 4, 3, 2, 6]);
        let node_3 = trie.get_child_from_route(vec![10, 7, 3]).unwrap();
        node_3.clone().borrow_mut().insert_child(3, node_3);
        trie.insert_route(vec![10, 7, 2, 7, 6]);
        trie.insert_route(vec![10, 7, 3, 2, 7, 6]);
        trie.insert_route(vec![6, 6, 9]);
        trie.insert_route(vec![9, 6, 9]);

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

        println!("\n=====\nParsing tokens: {:?}\n======\n", &tokens_iter);

        let mut reduce_count = 0 as u8;
        let mut incr_red_count = false;
        while cur_token.is_some() {
            let t = cur_token.unwrap();
            parse_stack.push(t);
            let mut res = self.reduce(parse_stack, reduce_count);
            while res.is_ok() {
                res = self.reduce(parse_stack, reduce_count);
                println!("Result: {:?}", res);
                incr_red_count = true;
            }
            if incr_red_count {
                reduce_count += 1;
                incr_red_count = false;
            }
            match res {
                Ok(r) => {
                },

                Err(t) => {
                    if t != "eof" {
                        panic!("Parsing error: {}", res.unwrap());
                    }
                },
            }
            cur_token = tokens_iter.next().cloned();
            println!("Cur Token: {:?}", cur_token.clone());
        }

        (parse_stack[0].parse_type == ParserTokenType::Func || parse_stack[0].parse_type == ParserTokenType::FuncList) && parse_stack.len() == 1
    }

    pub fn reduce<'a>(&self, mut parse_stack: &mut Vec<ParserToken<'a>>, reduce_idx: u8) -> Result<&'static str, &'static str> {
        // This is where we use the trie to follow our established rules
        //
        
        let mut rax = Err("failed to reduce {:?}"); 

        let mut cur_trie_node = self.trie.clone();
        let mut local_stack: Vec<u8> = vec![];
        for x in &mut *parse_stack {
            local_stack.push(x.parse_type.clone() as u8);

        }

        let mut i = 0;
        let mut w = 0;
        let mut j = i+w;
        while w < local_stack.len() {
        
            while j < local_stack.len() {
                println!("Iterated i-j, w: {}-{}, {}", &i, &j, &w);
                let slice = &local_stack[i..j+1];

                let mut res = cur_trie_node.get_child_from_route(slice.to_vec());

                rax = match res {
                    Some(s) => {
                        let leaf = s.borrow().get_leaf();

                        match leaf {
                            Some(n) => {    
                                let mut literal = String::new();
                                let mut parse_slice = &mut parse_stack[i..j+1].iter();

                                let mut cur_parse_slice = parse_slice.next();
                                while cur_parse_slice.is_some() {
                                    literal.push_str(cur_parse_slice.unwrap().literal);
                                    println!("Processing literal: {}", literal);
                                    cur_parse_slice = parse_slice.next();
                                }

                                (*parse_stack).drain(i..j+1); 
                                (*parse_stack).insert(i as usize, ParserToken { parse_type: from_u8(n), literal: literal.clone().leak()});

                                local_stack = vec![];
                                for x in &mut *parse_stack {
                                    local_stack.push(x.parse_type.clone() as u8);
                                }
                                i = 0;
                                w = 0;
                                j = i+w;

                                println!("parser_stack {:?}", parse_stack);
                                Ok("success")
                            },
                            None => Err("eof"),
                        }
                    },
                    None    => {Err("eof")}
                };

                i += 1;
                j = i+w;

            }

            w += 1;
            i = 0;
            j = i+w;
        }

        rax
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
pub struct ParserToken<'a> {
    parse_type: ParserTokenType,
    literal: &'a str,
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
