use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct TrieNode {
    options: Vec<(usize, Rc<RefCell<TrieNode>>)>,
}


impl TrieNode {
    pub fn new() -> Self {
        TrieNode { options: vec![] } 
    }

    pub fn insert_child(&mut self, idx: usize, node: Rc<RefCell<TrieNode>>) -> bool {
        self.options.push( (idx, node) );
        true
    }

    pub fn insert_node(&mut self, idx: usize) -> Rc<RefCell<TrieNode>> { 
        let node = Rc::from(RefCell::from(TrieNode::new()));
        self.options.push( (idx, node.clone()) );
        node.clone()
    }

    pub fn insert_route(&mut self, idx_s: Vec<usize>) -> bool {
        if idx_s.len() < 2 {
            return false;
        }

        let mut cur_node: Rc<RefCell<TrieNode>> = match self.has_child(idx_s[0]) {
            true =>  {
                self.get_child(idx_s[0]).expect("unreachable")
            },
            false => {
                self.insert_node(idx_s[0])
            },
        };
        for i in 1..idx_s.len() {
            let x = idx_s[i];
            if !cur_node.borrow_mut().has_child(x) {
                let clone = cur_node.borrow_mut().insert_node(x);
                cur_node = clone;
            } else {
                let clone = cur_node.borrow_mut().get_child(x);
                cur_node = clone.expect("this shouldn't happen");
            }
        }

        true
    }

    pub fn has_child(&self, idx: usize) -> bool {
        for x in &self.options {
            if x.0 == idx {
                return true;
            }
        }

        false
    }

    fn find_idx(&mut self, idx: usize) -> Option<Rc<RefCell<TrieNode>>> {
        for x in &mut self.options {
            if x.0 == idx {
                return Some(x.1.clone());
            }
        }

        None
    }

    pub fn get_child(&mut self, idx: usize) -> Option<Rc<RefCell<TrieNode>>> {
        if self.has_child(idx) {
            return self.find_idx(idx);
        } else {
            return None;
        }
    }

    pub fn get_leaf(&self) -> Option<usize> {

        if self.options.len() == 0 {
            return None;
        }

        let rcc = Rc::from(RefCell::from(self.clone()));
        for x in &self.options {
            {
                let will_loop = Rc::ptr_eq(&rcc, &x.1);
                if will_loop {
                    break;
                }
            }
            if x.1.borrow().clone().options.len() == 0 {
                return Some(x.0.clone());
            }
        }

        None
    }

    pub fn get_child_from_route(&mut self, idx_s: Vec<usize>) -> Option<Rc<RefCell<TrieNode>>> {
        if idx_s.len() < 1 {
            return None;
        }

        if !self.has_child(idx_s[0]) {
            return None;
        }

        let mut cur_node = self.get_child(idx_s[0]).expect("Did you expect this to work?");

        //
        //  suppose we have 1, 4, 1, 4, 8
        //
        //  it assigns cur_node -> 1
        //  then it checks our second idx, 2
        //  checks if 1 -> 2
        //  if it doesn't, return false, else:
        //      assign cur_node to 2
        //
        //      and repeat?

        for i in 1..idx_s.len() {
            let x = idx_s[i];
            if cur_node.borrow_mut().has_child(x) {
                let clone = cur_node.borrow_mut().get_child(x).unwrap();
                cur_node = clone;
            } else {
                return None;
            }
        }
        
        Some(cur_node)
    }
    
    //
    //  types: 1, 4
    //  types_ahead: 1, 4, 1
    //
    //  should return true false but it's not
    //
    //  len is not <= 0
    //
    //  cur_idx = 1
    //  child = Some(4)
    //
    //
    //  
    //
    //
    pub fn match_route(&mut self, idx_s: &Vec<usize>) -> bool {
        
        if idx_s.len() <= 0 {
            return false;
        }

        let mut cur_idx = 1;

        let mut child = self.get_child(idx_s[0]);
        while child.is_some() && cur_idx < idx_s.len() {
            if !child.clone().unwrap().borrow_mut().has_child(idx_s[cur_idx]) {
                return false;
            }

            child = child.unwrap().borrow_mut().get_child(idx_s[cur_idx]);
            
            cur_idx += 1;
        }

        cur_idx == idx_s.len()
    }

    pub fn get_route_child(&mut self, idx_s: &Vec<usize>) -> Option<Rc<RefCell<TrieNode>>> {
        
        let mut cur_idx = 0;

        let mut child = self.get_child(idx_s[cur_idx]);
        while child.is_some() && cur_idx < idx_s.len()-1 {
            cur_idx += 1;
            child = child.unwrap().borrow_mut().get_child(idx_s[cur_idx]);
        }

        child
    }
}
