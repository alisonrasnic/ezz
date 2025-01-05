struct TrieNode {
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

    pub fn has_child(&self, idx: u8) -> bool {
        for x in self.options {
            if x.0 == idx {
                return true;
            }
        }

        false
    }
}
