pub mod compiler_object;
pub mod ast;

use ast::*;

pub struct Compiler {
    calls: Vec<Box<Compilercall>>,
    vars:  Vec<Box<CompilerVar>>,
    objects: Vec<Box<dyn CompilerObject>>, 
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { vec![] } 
    }

    pub fn add_call(&mut self, name: &'static str, type_info: &'static str) {
        
    }

    pub fn compile(&mut self, parse_stack: Vec<Rc<RefCell<ParserToken>>) -> Vec<Rc<RefCell<ASTNode>>> {
        // TODO
    }
}
