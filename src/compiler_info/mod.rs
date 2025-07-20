use crate::parser::ParserTokenType;
use crate::ezz_type::EzzType;

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

    pub fn ezz_defaults() -> Vec<FnDef> {
        let mut rsx: Vec<FnDef> = vec![];

        rsx.push(FnDef::new("let", None, vec![Arg::new('i', ""), Arg::new('f', ""), Arg::new('v', "")], EzzType::Void, false));
        rsx.push(FnDef::new("let", None, vec![Arg::new('i', "")], EzzType::Void, false));
        
        rsx.push(FnDef::new("mut", None, vec![Arg::new('i', ""), Arg::new('f', ""), Arg::new('v', "")], EzzType::Void, false));
        rsx.push(FnDef::new("mut", None, vec![Arg::new('i', "")], EzzType::Void, false));

        rsx.push(FnDef::new("+", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::I32, true)); 
        rsx.push(FnDef::new("+", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::F32, true)); 
        rsx.push(FnDef::new("-", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::I32, true)); 
        rsx.push(FnDef::new("-", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::F32, true)); 
        rsx.push(FnDef::new("*", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::I32, true)); 
        rsx.push(FnDef::new("*", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::F32, true)); 
        rsx.push(FnDef::new("/", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::I32, true)); 
        rsx.push(FnDef::new("/", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::F32, true)); 

        rsx.push(FnDef::new("=", None, vec![Arg::new('i', ""), Arg::new('v', "")], EzzType::Void, true));

        rsx.push(FnDef::new("==", None, vec![Arg::new('i', ""), Arg::new('i', "")], EzzType::U1, true));
        rsx.push(FnDef::new("==", None, vec![Arg::new('i', ""), Arg::new('v', "")], EzzType::U1, true));
        rsx.push(FnDef::new("==", None, vec![Arg::new('v', ""), Arg::new('i', "")], EzzType::U1, true));
        rsx.push(FnDef::new("==", None, vec![Arg::new('v', ""), Arg::new('v', "")], EzzType::U1, true));

        rsx.push(FnDef::new("if", None, vec![Arg::new('v', ""), Arg::new('f', ""), Arg::new('v', "")], EzzType::Void, false));

        rsx
    }
    
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_type(&self) -> EzzType {
        self.fn_type.clone()
    }

    pub fn add_arg(&mut self, arg: Arg) {
        self.arguments.push(arg);
    }
}

