#[derive(PartialEq, Debug, Clone)]
pub struct TrieNode {
    options: Vec<(u8, TrieNode)>,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode { options: vec![] } 
    }

    pub fn insert_child(&mut self, idx: u8) -> bool {
        self.options.push( (idx, TrieNode::new()) );
        true
    }

    pub fn insert_route(&mut self, idx_s: Vec<u8>) -> bool {
        let mut cur_node = self;

        for x in idx_s {
            if !cur_node.has_child(x) {
                cur_node.insert_child(x);
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

    fn find_idx(&mut self, idx: u8) -> Option<&mut TrieNode> {
        for x in &mut self.options {
            if x.0 == idx {
                return Some(&mut x.1);
            }
        }

        None
    }

    pub fn get_child(&mut self, idx: u8) -> Option<&mut TrieNode> {
        if self.has_child(idx) {
            return self.find_idx(idx);
        } else {
            return None;
        }
    }
}
