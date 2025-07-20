use std::path::PathBuf;
use std::collections::HashMap;

use myl_tree::Tree;

use crate::parser::ParserToken;
use crate::tree_generator::TreeGenerator;
use crate::compiler_info::*;

pub struct CompilerContext {
    pub files: Vec<PathBuf>,
    pub tree:  Tree<ParserToken>,
    pub gen:   TreeGenerator,

    pub types: Vec<&'static str>,
    pub vars:  Vec<&'static str>,

    pub funcs: Vec<FnDef>,

    last_func: Option<usize>,
}

impl CompilerContext {

    pub fn new() -> Self {

        CompilerContext { files: vec![], tree: Tree::new(), gen: TreeGenerator::new(), types: vec![], funcs: FnDef::ezz_defaults(), vars: vec![], last_func: None }

    }

    pub fn ezz_default() -> Self {

        CompilerContext { files: vec![], tree: Tree::new(), gen: TreeGenerator::new(), types: vec!["i32", "u32", "u1", "str", "$", "f32", "char", "fn", "group"],
            funcs: FnDef::ezz_defaults(), 
            vars:  vec![],
            last_func: None,
        }
    }

    pub fn append_last_func(&mut self, arg: Arg) {
        if let Some(i) = self.last_func {
            self.funcs[i].add_arg(arg); 
        } else {
            panic!("Append to function but no function to append to");
        }
    }

    pub fn set_func(&mut self, def: FnDef) {
        if !self.funcs_has(&def) {
            println!("function added!");
            self.funcs.push(def);
            self.last_func = Some(self.funcs.len()-1);
        } else {
            panic!("UNIMPLEMENTED");
            // self.last_func = ??
        }
    }

    fn funcs_has(&self, def: &FnDef) -> bool {
        for x in &self.funcs {
            if x.get_name() == def.get_name() {
                return true;
            }
        }

        false
    }

    pub fn get_func<F: Fn(&FnDef) -> bool>(&self, f: F) -> Option<&FnDef> {
        for x in &self.funcs {
            if f(x) {
                return Some(x);
            }
        }

        None
    }
}
