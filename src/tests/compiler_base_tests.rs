use crate::compiler::*;
use crate::parser::{ParserToken, ParserTokenType};
use crate::CompilerContext;
use crate::compiler::Target::*;

use myl_tree::*;

use std::path::PathBuf;

#[test]
pub fn base_tests() {
    assert_eq!(true, true);

    let mut context = CompilerContext::ezz_default();
    context.files.push(PathBuf::from("put.ezz"));

    let mut head_token = TreeNode::new(ParserToken::new(ParserTokenType::FuncHeader, 0, 0, 0, 0));  

    let mut fn_token = TreeNode::new(ParserToken::new(ParserTokenType::Type, 0, 0, 2, 0));
    let mut id_token = TreeNode::new(ParserToken::new(ParserTokenType::Id, 0, 3, 6, 0));

    let mut str_token = TreeNode::new(ParserToken::new(ParserTokenType::Type, 0, 7, 10, 0));
    let mut inp_token = TreeNode::new(ParserToken::new(ParserTokenType::Id, 0, 11, 14, 0));

    let mut body_token = TreeNode::new(ParserToken::new(ParserTokenType::FuncBody, 0, 17, 49, 0));

    let mut api_token = TreeNode::new(ParserToken::new(ParserTokenType::Api, 0, 17, 20, 0));
    let mut append_asm_token = TreeNode::new(ParserToken::new(ParserTokenType::Id, 0, 23, 34, 0));
    let mut fasm_token = TreeNode::new(ParserToken::new(ParserTokenType::Str, 0, 35, 147, 0));

    let mut value_token = TreeNode::new(ParserToken::new(ParserTokenType::Num, 0, 150, 151, 0));

    let mut tree = Tree::<ParserToken>::new();

    head_token.set_left(&mut fn_token);
    fn_token.set_left(&mut str_token);
    str_token.set_left(&mut inp_token);

    head_token.set_right(&mut body_token);
    body_token.set_left(&mut api_token);
    api_token.set_left(&mut append_asm_token);
    append_asm_token.set_left(&mut fasm_token);

    body_token.set_right(&mut value_token);

    tree.set_head(&mut body_token);

    let mut compiler = Compiler::new(CompilerTarget::FasmWinx86);

    compiler.compile_body(&mut tree.get_head().unwrap(), &mut context);

    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create("a.asm");
    file.unwrap().write_all(&compiler.get_text().clone().into_bytes());
}
