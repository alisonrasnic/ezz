
use crate::trie::TrieNode;
use crate::tree_generator::TreeGenerator;
use crate::compiler_context::CompilerContext;
use crate::compiler_info::*;

use myl_tree::{Tree, TreeNode};

use std::path::PathBuf;

/// Contains RegEx Trie's for grammar rules
/// 
/// ``parse`` function is the start point for parsing tokens from Lexer into ``TreeGenerator``
///
/// Parser does two passes over tokens passed to it.
/// # First Pass
/// The first pass reads headers of functions it encounters, and skips the body of the functions.
/// This is done so that when functions are called within a body, we already know what type it is,
/// what arguments it expects to receive, whether it's in scope or not, and can be called even if
/// the function is defined after the function calling it.
/// # Second Pass
/// The second pass reads the bodies of the function headers already processed. 
#[derive(Debug, PartialEq, Clone)]
pub struct Parser {
    trie_1: TrieNode,
    trie_2: TrieNode,
}

impl Parser {
    pub fn new() -> Parser {
        let mut trie_1 = TrieNode::new();
        let mut trie_2 = TrieNode::new();
        trie_1.insert_route(vec![1, 4, 10]);

        trie_1.insert_route(vec![10, 1, 4, 18]);        
        trie_1.insert_route(vec![18, 8, 1, 4, 19]);
        trie_1.insert_route(vec![19, 8, 1, 4, 19]);

        trie_1.insert_route(vec![10, 20]);
        trie_1.insert_route(vec![18, 20]);
        trie_1.insert_route(vec![19, 20]);

        // 6 represents a fully defined function and therefore, does not need to be in the parse
        //   stack
        trie_2.insert_route(vec![1, 4, 10]);

        trie_2.insert_route(vec![20, 17, 20]);
        trie_2.insert_route(vec![20, 6, 23]);
        trie_2.insert_route(vec![20, 0, 23]);
        trie_2.insert_route(vec![23, 0, 13, 23]);
        trie_2.insert_route(vec![23, 6, 23]);
        trie_2.insert_route(vec![23, 4, 23]);
        trie_2.insert_route(vec![23, 15, 23]);
        trie_2.insert_route(vec![23, 16, 23]);
        trie_2.insert_route(vec![23, 4, 8, 4, 23]);
        trie_2.insert_route(vec![23, 13, 23]);

        Parser { trie_1: trie_1, trie_2: trie_2 }
    }

    // lex is necessary to convert from string into a token
    //  as opposed to the parse which changes tokens into simpler tokens

    pub fn parse(&mut self, tokens: Vec<ParserToken>, context: &mut CompilerContext, parse_stack: &mut Vec<ParserToken>) {
        let mut cur_tok_idx: usize = 0;

        while cur_tok_idx < tokens.len() {
            parse_stack.push(tokens[cur_tok_idx].clone());
            println!("\n\nParse Stack: {:?}\n\n", &parse_stack);
            let mut res = self.full_reduce_1(context, parse_stack);

            if let Err(v) = res {
                if v == "Unknown" {
                    self.step(&tokens, &mut cur_tok_idx);
                } else if v == "Skip" {
                    *parse_stack = vec![];
                } else if v == "Retry" {
                } else {
                    panic!("ANSDKLA");
                }
            }
        }

        *parse_stack = vec![];

        cur_tok_idx = 0;
        println!("\n\n\nBeginning pass 2 of parsing...\n\n\n");
        while cur_tok_idx < tokens.len() {
            parse_stack.push(tokens[cur_tok_idx].clone());

            let mut res = self.full_reduce_2(context, parse_stack);

            if let Err(v) = res {
                if v == "Unknown" {
                    self.step(&tokens, &mut cur_tok_idx);
                } else if v.starts_with("Skip") {
                    let num = (v.split(' ').collect::<Vec<&'static str>>()[1].chars().nth(0).unwrap() ).to_digit(10).unwrap();
                    println!("Num to skip: {}", num);
                    let id = &parse_stack[0].get_id(); 
                    let start = &parse_stack[0].get_start(); 
                    let end = &parse_stack[0].get_end(); 
                    let line = &parse_stack[0].get_line();
                    *parse_stack = vec![];
                    for i in 0..num {
                        println!("Skipping...");
                        self.step(&tokens, &mut cur_tok_idx);
                    }

                    let fndef = ParserToken::new(ParserTokenType::FuncHeader, *id, *start, *end, *line);
                    parse_stack.push(fndef);
                } else if v == "Retry" {
                } else {
                    panic!("Error: {}", v);
                }
            }
        }

        parse_stack.pop();
    }

    pub fn step(&self, tokens: &Vec<ParserToken>, cur_token: &mut usize) -> Result<&'static str, &'static str> 
    {
        // step simply moves cur_token to our next token
        *cur_token += 1;

        Ok("Success")
    }


    /*
     *     This first pass aims to parse every function header so we can pass over again and then
     *     fully process the function bodies, allowing for functions to be defined after they are
     *     used as well as being able to distinguish accurately between functions, variables, and
     *     other scoping issues.
     *
     *     We already have it processing function headers so how do we get it to only process the
     *     function headers without anything else?
     *
     *     Consider the following:
     *       fn hi 
     *         put "Hi"
     *         0
     *       
     *       i32 mn$args
     *         let x = sizeOf i32
     *         hi
     *         x
     *
     *    The problem here if we try to only parse FnHeaders is that types can be an "object" at
     *    times. we have Type, Id and then the current parser will think that there is a fn header called sizeOf that
     *    takes an i32 and then returns that function.
     *
     *    The real question out of this is how do we handle "type objects"?
     *
     *    Solution 1:
     *      Do not allow any type objects.
     *      
     *      In this case, measuring memory size of objects would have to be done a different way.
     *    
     *    Solution 2:
     *      Type objects MUST be inside a group
     *
     *      i.e. :
     *        i32 mn$args
     *          let x = sizeOf[i32]
     *          hi
     *          x
     *        
     *      hopefully here, the parser would see the identifier sizeOf, and then see a group with a
     *      type inside and nothing else, this implies it is a type object. this would also
     *      requires us to not support function headers inside of groups, however.
     *
     *      Groups fundamentally are for separation/clarification, but in a sense, is a scoping
     *      mechanism. e.g. :
     *                          i32 mn$args
     *                            let x = [fn sizeOf [type] typ /*code here*/]
     *                            x i32 
     *                            0
     *
     *      I think this is fine??
     *
     */
    pub fn full_reduce_1(&mut self, context: &mut CompilerContext, parse_stack: &mut Vec<ParserToken>) -> Result<&'static str, &'static str> {

        if parse_stack.len() == 0 {
            return Err("Retry");
        }
        /*
         *
         *    LR(1) parsing
         *
         *    check if our current stack matches a route in trie
         *    if it does, and adding one more to the stack makes it not match anymore
         *    then a reduction is made.
         *
         */

        // assume parse_stack contains 1 look-ahead symbol 

        let path = &context.files[parse_stack[0].id];
        let res = self.func_reduce(&parse_stack[0..parse_stack.len()-1], &parse_stack[parse_stack.len()-1], &path);

        if let Err(msg) = res {

            // this means we need to step
            if msg == "Unknown" {
                return Err("Unknown");
            } else if msg == "Skip" {
                return Err("Skip");
            } else {
                // this means an actual error has occurred
                panic!("{}", msg);
            }

        } else {
            // this means the reduction was successful and we need to gen a new token
            // this also means we need to edit the parse stack 

            let s = res.unwrap();
            let id = &parse_stack[0].id;
            let start = &parse_stack[0].start;
            let end   = &parse_stack.last().unwrap().end;
            let line  = &parse_stack[0].line;
            let new_token = ParserToken::new(s, *id, *start, *end, *line);

            let last = parse_stack.last().unwrap().clone();
            if s != ParserTokenType::FuncHeader && s != ParserTokenType::FuncHeaderNArg && s != ParserTokenType::FuncHeaderArgs && s != ParserTokenType::FuncHeaderMArgs {
                *parse_stack = vec![];
                parse_stack.push(new_token);
            } else if s == ParserTokenType::FuncHeaderNArg {

                /*
                 *      first token is our type
                 *      second token is our name
                 */

                use crate::ezz_type::*;
                let mut typ = str_to_type(parse_stack[0].get_literal(&context.files[new_token.get_id()]));
                let mut def = FnDef::new((&parse_stack[1]).get_literal(&context.files[new_token.get_id()]), Some(*start), vec![], typ, false);
                context.set_func(def);

                {
                    context.gen.string_tree_1(&mut parse_stack[0..parse_stack.len()-1].to_vec(), true);
                }

                *parse_stack = vec![];
                parse_stack.push(new_token);
            
            } else if s == ParserTokenType::FuncHeaderArgs {
                /*
                 *   this means  ?
                 *   we append an arg but how do we remember what fn to append to
                 *      1: we could append to our last created fn
                 *
                 */

                use crate::ezz_type::*;
                let mut literal = parse_stack[2].get_literal(&context.files[new_token.get_id()]);
                let mut literal_type = parse_stack[1].get_literal(&context.files[new_token.get_id()]);
                let mut arg_type = str_to_type(literal_type);

                let mut arg = Arg::from_type(arg_type, literal); 
                context.append_last_func(arg);

                context.gen.add_leaf_to_string(&mut parse_stack[1], true);
                context.gen.add_leaf_to_string(&mut parse_stack[2], true);

                *parse_stack = vec![];
                parse_stack.push(new_token);

            } else if s == ParserTokenType::FuncHeaderMArgs {
                /*
                 *   this means  ?
                 *   we append an arg but how do we remember what fn to append to
                 *      1: we could append to our last created fn
                 *
                 */

                use crate::ezz_type::*;
                let mut literal = parse_stack[3].get_literal(&context.files[new_token.get_id()]);
                let mut literal_type = parse_stack[2].get_literal(&context.files[new_token.get_id()]);

                let mut arg = Arg::from_type(str_to_type(literal_type), literal); 
                context.append_last_func(arg);
                
                context.gen.add_leaf_to_string(&mut parse_stack[1], true);
                context.gen.add_leaf_to_string(&mut parse_stack[2], true);

                *parse_stack = vec![];
                parse_stack.push(new_token);

            } else { 
                *parse_stack = vec![];
            }

            //parse_stack.push(last);

            return Ok("Success");

        }

        Err("")
    }

    pub fn full_reduce_2(&mut self, context: &mut CompilerContext, parse_stack: &mut Vec<ParserToken>) -> Result<&'static str, &'static str> {

        if parse_stack.len() == 0 {
            return Err("Unknown");
        }

        // pass 2
        //
        //      here we need to ignore function headers, potentially a tricky topic. or at least if
        //      encountered, find their name and skip the appropriate amount of tokens. but this
        //      depends on which definition of it we have.
        //
        //      we could also record the position of each function definition and then if our token
        //      starts at that, we know exactly how many tokens to skip
        //
        //      then we can read the arguments into our vars list and process the body
        //      if correct, it passes out of the parse_stack and into the tree.
        //
        //      when the function returns its last value, all of its variables and arguments are
        //      removed from our variables, maintaining scope.
        //

        let mut path = &context.files[parse_stack[0].id];

        let mut res = self.reduce(&parse_stack[0..parse_stack.len()-1], &parse_stack[parse_stack.len()-1], path);

        if let Ok(s) = res {
            // s is our returned new token type
            // so for example, if we encounter a FuncHeaderNArg, then we need to look it up and
            // skip ahead

            use crate::ezz_type::*;
            let func = &context.get_func(|x| (*x).get_type() == str_to_type(parse_stack[0].get_literal(path)) && x.get_name() == parse_stack[1].get_literal(path));

            if let Some(f) = func {
                return Err(format!("Skip {}", (*f).0.get_args().len()*2).leak());
            } else {

                /*
                 *
                 *      this is where we interpret function bodies into abstract syntax trees
                 *
                 */

                let id = &parse_stack[0].get_id();
                let start = &parse_stack[0].get_start();
                let end = &parse_stack[0].get_end();
                let line = &parse_stack[0].get_line();
                let new_token = ParserToken::new(s, *id, *start, *end, *line);
                *parse_stack = vec![];

                if s == ParserTokenType::FuncHeader {
                    return Ok("Success");
                } else if s == ParserTokenType::Api {
                    return Ok("Skip 2");
                } else {
                    parse_stack.push(new_token);
                    return Ok("Success");
                }
            }

        } else {
            return Err("Unknown");
        }

        Ok("Success")
    }

    fn reduce(&mut self, slice: &[ParserToken], look_ahead: &ParserToken, path: &PathBuf) -> Result<ParserTokenType, &'static str> {
        // reduce does a single reduce of a stack of tokens
        let mut types: Vec<usize> = vec![];
        for t in slice {
            types.push(t.get_type() as usize);
        }

        if types.len() == 0 {
            return Err("Unknown");
        }

        let mut types_ahead = types.clone();
        types_ahead.push(look_ahead.get_type() as usize);

        let res = self.trie_2.match_route(&types);
        let res_la = self.trie_2.match_route(&types_ahead);

        if res {
            if !res_la {
                if let Some(s) = self.get_regex_2(types) {
                    return Ok(s);
                } else {
                    return Err("Unknown");
                }
            }

            return Err("Unknown");
        } else {
            return Err("Failed to reduce");
        }
    }

    fn func_reduce(&mut self, slice: &[ParserToken], look_ahead: &ParserToken, path: &PathBuf) -> Result<ParserTokenType, &'static str> {
        // reduce does a single reduce of a stack of tokens
        let mut types: Vec<usize> = vec![];
        for t in slice {
            types.push(t.get_type() as usize);
        }

        if types.len() == 0 {
            return Err("Unknown");
        }

        let mut types_ahead = types.clone();
        types_ahead.push(look_ahead.get_type() as usize);

        let res = self.trie_1.match_route(&types);
        let res_la = self.trie_1.match_route(&types_ahead);

        if res {
            if !res_la {
                let regex = self.get_regex(types);
                if let Some(t) = regex {
                    return Ok(t);
                } else {
                    return Err("Skip");
                }
            }

            return Err("Unknown");
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
    fn get_regex(&mut self, vals: Vec<usize>) -> Option<ParserTokenType> {
        use std::rc::Rc;
        let r = self.trie_1.get_child_from_route(vals);

        if let Some(ref s) = r {
            if let Ok(trie_node) = Rc::try_unwrap(s.into()) {
                if let Some(val) = trie_node.borrow_mut().get_leaf() {
                    return Some(ParserTokenType::from_usize(val));
                }
            } else {
                panic!("Rc unwrap failure: {:?}", r.unwrap().borrow() );
            }
        } else {
        }

        None
    }

    fn get_regex_2(&mut self, vals: Vec<usize>) -> Option<ParserTokenType> {
        use std::rc::Rc;
        let r = self.trie_2.get_child_from_route(vals);

        if let Some(ref s) = r {
            if let Ok(trie_node) = Rc::try_unwrap(s.into()) {
                if let Some(val) = trie_node.borrow_mut().get_leaf() {
                    return Some(ParserTokenType::from_usize(val));
                }
            } else {
                panic!("Rc unwrap failure: {:?}", r.unwrap().borrow() );
            }
        } else {
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

    pub fn get_id(&self) -> usize {
        self.id
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
    Api=0,
    Expr=3,
    Func=6,
    Id=4,
    Op=5,
    Type=1,
    Delim=7,
    Comma=8,
    FuncList=9,
    FuncHeaderNArg=10,
    Declare=11,
    Assignment=12,
    Str=13,
    Ch=14,
    Num=15,
    Float=16,
    Bool=17,
    FuncHeaderArgs=18,
    FuncHeaderMArgs=19,
    FuncHeader=20,
    Group=21,
    Void=22,
    FuncBody=23,
}

impl ParserTokenType {
    pub fn from_usize(num: usize) -> ParserTokenType {
        match num {
            0 => ParserTokenType::Api,
            1 => ParserTokenType::Type,
            3 => ParserTokenType::Expr,
            4 => ParserTokenType::Id,
            5 => ParserTokenType::Op,
            6 => ParserTokenType::Func,
            7 => ParserTokenType::Delim,
            8 => ParserTokenType::Comma,
            9 => ParserTokenType::FuncList,
            10 => ParserTokenType::FuncHeaderNArg,
            11 => ParserTokenType::Declare,
            12 => ParserTokenType::Assignment,
            13 => ParserTokenType::Str,
            14 => ParserTokenType::Ch,
            15 => ParserTokenType::Num,
            16 => ParserTokenType::Float,
            17 => ParserTokenType::Bool,
            18 => ParserTokenType::FuncHeaderArgs,
            19 => ParserTokenType::FuncHeaderMArgs,
            20 => ParserTokenType::FuncHeader,
            21 => ParserTokenType::Group,
            22 => ParserTokenType::Void,
            23 => ParserTokenType::FuncBody,
            _ => ParserTokenType::Id,
        }
    }

    pub fn is_value(&self) -> bool {
       (*self as usize) >= 13 && (*self as usize) <= 17
    }

    pub fn is_fn_head(&self) -> bool {
        (*self as usize) == 20
    }
}
