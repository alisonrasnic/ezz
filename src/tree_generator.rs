
use myl_tree::{TreeNode, Tree};
use crate::parser::ParserToken;
use std::mem::*;

pub struct TreeGenerator {
    
    nodes: Vec<TreeNode<ParserToken>>,
    tree: Tree<ParserToken>,

}

impl TreeGenerator {
    pub fn new() -> Self {

        TreeGenerator { nodes: vec![], tree: Tree::new() }

    }

    pub fn take(&mut self, token: ParserToken) -> &TreeNode<ParserToken> {

        let mut node = TreeNode::new(token);
        self.nodes.push(node);
        //forget(node);

        self.nodes.last().unwrap()

    }

    pub fn take_mut(&mut self, token: ParserToken) -> &mut TreeNode<ParserToken> {

        let mut node = TreeNode::new(token);
        self.nodes.push(node);

        self.nodes.last_mut().unwrap()
    }
}
