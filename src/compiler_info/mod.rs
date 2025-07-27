use crate::parser::{ParserToken, ParserTokenType};
use crate::ezz_type::EzzType;
use myl_tree::*;

#[derive(Debug, PartialEq)]
pub struct Arg {
    arg_type: EzzType,
    arg_id: &'static str,
}

#[derive(Debug, PartialEq)]
pub struct FnDef {
    name: &'static str,
    start: Option<usize>,
    arguments: Vec<Arg>,
    fn_type:   EzzType,
    prefix: bool,
}

impl Arg {
    pub fn new(typ: char, id: &'static str) -> Self {
        match typ {
            'v' => Arg { arg_type: EzzType::Value, arg_id: id },
            'f' => Arg { arg_type: EzzType::Void, arg_id: id },
            _ => Arg { arg_type: EzzType::Identifier, arg_id: id },
        }
    }

    pub fn from_type(typ: EzzType, id: &'static str) -> Self {
        Arg { arg_type: typ, arg_id: id }
    }
}

impl FnDef {
    pub fn new(name: &'static str, start: Option<usize>, args: Vec<Arg>, typ: EzzType, prefix: bool) -> Self {
        FnDef { name: name, start: start, arguments: args, fn_type: typ, prefix: prefix }
    }

    fn let_tree() -> Tree<ParserToken> {
        let tree = Tree::<ParserToken>::new();

        tree
    }

    pub fn ezz_defaults() -> Vec<(FnDef, Option<Tree<ParserToken>>)> {
        let mut rsx: Vec<(FnDef, Option<Tree<ParserToken>>)> = vec![];

        rsx.push((FnDef::new("let", None, vec![Arg::new('i', ""), Arg::new('f', ""), Arg::new('v', "")], EzzType::Identifier, false), None));
        rsx.push((FnDef::new("let", None, vec![Arg::new('i', "")], EzzType::Identifier, false), None));
        
        rsx.push((FnDef::new("mut", None, vec![Arg::new('i', ""), Arg::new('f', ""), Arg::new('v', "")], EzzType::Identifier, false), None));
        rsx.push((FnDef::new("mut", None, vec![Arg::new('i', "")], EzzType::Identifier, false), None));

        rsx.push((FnDef::new("+", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::I32, true), None)); 
        rsx.push((FnDef::new("+", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::F32, true), None)); 
        rsx.push((FnDef::new("-", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::I32, true), None)); 
        rsx.push((FnDef::new("-", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::F32, true), None)); 
        rsx.push((FnDef::new("*", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::I32, true), None)); 
        rsx.push((FnDef::new("*", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::F32, true), None)); 
        rsx.push((FnDef::new("/", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::I32, true), None)); 
        rsx.push((FnDef::new("/", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::F32, true), None)); 

        rsx.push((FnDef::new("=", None, vec![Arg::new('i', ""), Arg::new('v', "")], EzzType::Void, true), None));

        rsx.push((FnDef::new("==", None, vec![Arg::new('i', ""), Arg::new('i', "")], EzzType::U1, true), None));
        rsx.push((FnDef::new("==", None, vec![Arg::new('i', ""), Arg::new('v', "")], EzzType::U1, true), None));
        rsx.push((FnDef::new("==", None, vec![Arg::new('v', ""), Arg::new('i', "")], EzzType::U1, true), None));
        rsx.push((FnDef::new("==", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::U1, true), None));

        rsx.push((FnDef::new("if", None, vec![Arg::new('v', ""), Arg::new('f', ""), Arg::new('v', "")], EzzType::Void, false), None));

        rsx
    }
    
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_type(&self) -> EzzType {
        self.fn_type.clone()
    }

    pub fn get_args(&self) -> &Vec<Arg> {
        &self.arguments
    }

    pub fn add_arg(&mut self, arg: Arg) {
        self.arguments.push(arg);
    }
}

