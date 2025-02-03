
use crate::trie::TrieNode;
use crate::tree::TreeNode;
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

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

    pub fn parse(&self, tokens: Vec<ParserToken>, parse_stack: &mut Vec<ParserToken>) -> Rc<RefCell<TreeNode>> {
        let mut cur_token: Option<ParserToken> = None;
        let mut tokens_iter = tokens.iter();
        cur_token = tokens_iter.next().cloned();

        let mut ast: Rc<RefCell<TreeNode>> = Rc::from(RefCell::from(TreeNode::new((cur_token.clone().unwrap()))));
        let mut ast_head: Rc<RefCell<TreeNode>> = ast.clone();

        let mut reduce_count = 0 as u8;
        let mut incr_red_count = false;
        while cur_token.is_some() {
            let t = cur_token.clone().unwrap();
            parse_stack.push(t);
            
            let mut res = self.reduce(parse_stack, &mut ast, &mut ast_head, reduce_count);
            while res.is_ok() {
                res = self.reduce(parse_stack, &mut ast, &mut ast_head, reduce_count);
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

            ast_head.borrow().vlr_print(true);
            println!("\n---------------------------------------------------------------\n");

            cur_token = tokens_iter.next().cloned();

            if cur_token.is_some() {
                let new_node = Rc::from(RefCell::from(TreeNode::new((cur_token.clone().unwrap()))));
                ast.borrow_mut().set_left(new_node.clone());
                ast = new_node;
            } else {
                break; 
            }
        }

        ast_head.clone()
        //(parse_stack[0].parse_type == ParserTokenType::Func || parse_stack[0].parse_type == ParserTokenType::FuncList) && parse_stack.len() == 1
    }

    pub fn reduce(&self, mut parse_stack: &mut Vec<ParserToken>, ast: &mut Rc<RefCell<TreeNode>>, ast_head: &mut Rc<RefCell<TreeNode>>, reduce_idx: u8) -> Result<&'static str, &'static str> {
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
                                    literal.push_str(&cur_parse_slice.unwrap().literal);
                                    cur_parse_slice = parse_slice.next();
                                }

                                println!("\nSearching for: {:?}\n", parse_stack[i].clone());
                                let mut bfs = ast_head.borrow().search(parse_stack[i].clone());

                                /*
                                 *
                                 *
                                 *      we need to find the root node
                                 *      then reposition our new node there and hook up our old node
                                 *      to that new node
                                 *
                                 *
                                 */
                                if let Some(mut v) = bfs {
                                    let new_v = Rc::from(RefCell::from(TreeNode::new(ParserToken { parse_type: from_u8(n), literal: literal.clone()})));
                                    new_v.borrow_mut().set_right(v.clone());
                                   

                                    let mut reduce_parent = ast_head.borrow_mut().search_for_parent_of(parse_stack[i].clone());
                                    /*
                                     *
                                     *      the ast_head in here needs to be replaced with
                                     *      essentially the parent of the start point. sometimes we
                                     *      are not looking for the behavior of this.
                                     *
                                     *      maybe we can code a search_for_parent_of ?
                                     *
                                     */

                                    if v.borrow().get_value() == ast_head.borrow().get_value() {
                                        println!("!! reassigning head to: {:?}", new_v.clone());
                                        *ast_head = new_v.clone();
                                    } else {
                                        
                                        if reduce_parent.is_some() {
                                            let mut parent = reduce_parent.as_mut().unwrap();
                                            println!("\nPARENT IS: {:?} || INSERTING INTO LEFT: {:?}\n", parent, new_v.clone());
                                            if Rc::ptr_eq(&parent, &ast_head) {
                                                ast_head.borrow_mut().set_left(new_v.clone());
                                            } else {
                                                parent.borrow_mut().set_left(new_v.clone());
                                            }
                                            println!("\nPARENT IS: {:?}\n", parent);
                                        } else {
                                            panic!("Should be unreachable (v): {:?}", reduce_parent.clone());
                                        }
                                    }

                                    println!("\n\n\n\nAST_HEAD AS OF NOW:\n\n\n\n");
                                    ast_head.borrow().vlr_print(true);
                                    println!("\n\n\n\n\n\n\n");

                                    //bfs = Some(new_v.clone());

                                    *ast = new_v.clone();
                                } else {
                                    println!("Should be unreachable (v): {:?}", bfs.clone());
                                }

                                (*parse_stack).drain(i..j+1); 
                                (*parse_stack).insert(i as usize, ParserToken { parse_type: from_u8(n), literal: literal.clone()});
                                
                                println!("\nReducing to token: {:?}\n", ParserToken { parse_type: from_u8(n), literal: literal.clone()});

                                local_stack = vec![];
                                for x in &mut *parse_stack {
                                    local_stack.push(x.parse_type.clone() as u8);
                                }
                                i = 0;
                                w = 0;
                                j = i+w;

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
