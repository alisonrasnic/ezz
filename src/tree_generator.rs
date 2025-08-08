
use myl_tree::{TreeNode, Tree};
use crate::parser::ParserToken;
use std::mem::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TreeGenerator {
    
    nodes: Vec<TreeNode<ParserToken>>,
    tree: Tree<ParserToken>,

}

impl<'a> TreeGenerator {
    pub fn new() -> Self {

        TreeGenerator { nodes: vec![], tree: Tree::new() }

    }

    pub fn get_tree(&self) -> &Tree<ParserToken> {
        &self.tree
    }

    pub fn take(&mut self, token: ParserToken) -> &TreeNode<ParserToken> {

        let mut node = TreeNode::new(token);
        self.nodes.push(node);

        self.nodes.last().unwrap()

    }

    pub fn take_mut(&mut self, token: ParserToken) -> &mut TreeNode<ParserToken> {

        let mut node = TreeNode::new(token);
        self.nodes.push(node);

        self.nodes.last_mut().unwrap()
    }

    pub fn rehead(&mut self, token: usize, left: bool) {
        self.tree.rehead(self.nodes.get_mut(token).unwrap(), left);
    }
    
    pub fn find_node(&mut self, token: &mut ParserToken) -> Option<usize> {
        // find node in self.nodes by every field being equivalent
        //
        // if not found return None

        for i in 0..self.nodes.len() {
            let n = &self.nodes[i];
            let elem = n.get_elem();
            if elem.get_id() == token.get_id() &&
                elem.get_start() == token.get_start() &&
                elem.get_end() == token.get_end() &&
                elem.get_line() == token.get_line() {
                
                return Some(i);
            }
        }

        None
    }

    // this is the code for stringing a tree together in pass 1 of the parser
    pub fn string_tree_1(&mut self, vec: &mut Vec<ParserToken>, left: bool) {
        println!("Hello there");
        if vec.len() <= 0 {
            panic!("Called string_tree on empty/invalid vector");
        }
        
        let mut idx: Vec<usize> = vec![];

        let mut cur_token = vec.pop();
        while let Some(mut t) = cur_token {
            if let Some(n) = self.find_node(&mut t) {
                idx.push(n);
            }

            cur_token = vec.pop();
        }

        if idx.len() <= 0 {
            eprintln!("tree_generator.rs: string_tree called but no nodes from vec found in self.nodes");
            return;
        }

        for i in idx {
            self.rehead(i, true);
        }
    }

    // this is the code for stringing a tree together in pass 2 of the parser
    pub fn string_tree_2(&mut self, vec: &mut Vec<ParserToken>, left: bool) {
        // TODO: Check if our first node is a FuncHeader, if not, we have a problem
        //
        // then parse the rest as an fnbody node tree that will be the right node of the fnheader
        // first scanned
    }

    pub fn add_leaf_to_string(&mut self, token: &mut ParserToken, left: bool) {
        // here we are adding a leaf to the bottom of the tree 
        // how do we know where to add it?
        //
        // finding the left bottom is the end of our relevant string in this case.
        //
        // for example:
        //                  FuncHeaderArgs
        //              mn
        //          i32
        //      args
        //    $
        //
        panic!("TODO");
    }
}
