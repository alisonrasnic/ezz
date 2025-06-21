use std::path::PathBuf;
use std::collections::HashMap;

use myl_tree::Tree;

use crate::parser::ParserToken;
use crate::tree_generator::TreeGenerator;

pub struct CompilerContext {
    pub files: Vec<PathBuf>,
    pub tree:  Tree<ParserToken>,
    pub gen:   TreeGenerator,

    pub types: Vec<&'static str>,
    pub funcs: Vec<&'static str>,
    pub vars:  Vec<&'static str>,

    type_funcs: HashMap<&'static str, &'static str>,
}

impl CompilerContext {

    pub fn new() -> Self {

        CompilerContext { files: vec![], tree: Tree::new(), gen: TreeGenerator::new(), types: vec![], funcs: vec![], vars: vec![], type_funcs: HashMap::new() }

    }

    pub fn ezz_default() -> Self {

        CompilerContext { files: vec![], tree: Tree::new(), gen: TreeGenerator::new(), types: vec!["i32", "u32", "u1", "str", "$", "f32", "char", "fn"],
            funcs: vec!["let", "mut", "put", "while", "if", "elif", "else", "for", "match", "+", "=", "-", "/", "*"],
            vars:  vec![],
            type_funcs: HashMap::new()
        }

    }
}
