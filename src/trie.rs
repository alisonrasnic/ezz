#[derive(PartialEq, Debug, Clone)]
pub struct TrieNode {
    options: Vec<(u8, Rc<TrieNode>)>,
}

use std::rc::Rc;

impl TrieNode {
    pub fn new() -> Self {
        TrieNode { options: vec![] } 
    }

    pub fn insert_node(&mut self, idx: u8, node: Rc<TrieNode>) -> bool {
        self.options.push( (idx, node) );
        true
    }

    pub fn insert_route(&mut self, idx_s: Vec<u8>, nodes: Vec<Rc<TrieNode>>) -> bool {
        let mut cur_node = Rc::from(self);

        for i in 0..idx_s.len() {
            let x = idx_s[i];
            let x_node = nodes[i];
            if !cur_node.has_child(x) {
                cur_node.insert_node(x, x_node);
            }

            let mut new_node = cur_node.find_idx(x);

            match new_node {
                Some(n) => {cur_node = n;},
                None    => {panic!("oopsi");},
            };
        }
        
        true
    }

    pub fn has_child(&self, idx: u8) -> bool {
        for x in &self.options {
            if x.0 == idx {
                return true;
            }
        }

        false
    }

    fn find_idx(&self, idx: u8) -> Option<Rc<&mut TrieNode>> {
        for x in &self.options {
            if x.0 == idx {
                return Some(x.1.clone());
            }
        }

        None
    }

    pub fn get_child(&mut self, idx: u8) -> Option<Rc<&mut TrieNode>> {
        if self.has_child(idx) {
            return self.find_idx(idx);
        } else {
            return None;
        }
    }

    pub fn get_child_from_route(&mut self, idx_s: Vec<u8>) -> Option<Rc<&mut TrieNode>> {
        let mut cur_node = Rc::from(self);

        for x in idx_s {
            if cur_node.has_child(x) {
                cur_node = cur_node.get_child(x).unwrap();
            } else {
                return None;
            }
        }
        
        Some(cur_node)
    }
}
