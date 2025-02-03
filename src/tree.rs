use std::cell::RefCell;
use std::rc::Rc;

use crate::parser::ParserToken;

#[derive(PartialEq, Debug, Clone)]
pub struct TreeNode {
    value: ParserToken,
    left:  Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

use std::any::Any;

impl TreeNode {
    pub fn new(token: ParserToken) -> Self {
        TreeNode { value: token, left: None, right: None } 
    }

    pub fn get_value(& self) -> & ParserToken {
        &self.value
    }

    pub fn get_left(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.right.clone()
    }

    pub fn set_left(&mut self, token: Rc<RefCell<TreeNode>>) {
        self.left = Some(token);
    }

    pub fn set_right(&mut self, token: Rc<RefCell<TreeNode>>) {
        self.right = Some(token);
    }

    pub fn search(&self, val: ParserToken) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sub_rax1: Option<Rc<RefCell<TreeNode>>> = None;
        let mut sub_rax2: Option<Rc<RefCell<TreeNode>>> = None;
        if self.value == val {
            return Some(Rc::from(RefCell::from(self.clone())));
        } else {
            sub_rax1 = self.l_bfs(val.clone());
            sub_rax2 = self.r_bfs(val.clone());
        }

        if sub_rax1.is_some() {
            return sub_rax1;
        }

        sub_rax2
    }

    fn l_bfs(&self, val: ParserToken) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(left) = &self.left {
            let bor = left.borrow();
            return bor.search(val);
        } else {
            return None;
        }
    }

    fn r_bfs(&self, val: ParserToken) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(right) = &self.right {
            let bor = right.borrow();
            return bor.search(val);
        } else {
            return None;
        }
    }

    /// !!!!!! IMPORTANT 
    ///
    /// THIS FUNCTION IS CURRENTLY RETURNING DUPLICATE NODES INSTEAD OF THE ORIGINALS
    pub fn search_for_parent_of(&mut self, val: ParserToken) -> &mut Option<Rc<RefCell<TreeNode>>> {
        panic!("TODO: FIX DUPLICATING IN THIS FN");
        let mut rax:  Option<Rc<RefCell<TreeNode>>> = None;

        if self.value == val {
            return &mut None;
        } else {
            rax = self.parent_search_l(val.clone());
            if rax.is_none() {
                rax = self.parent_search_r(val.clone());
            }
        }

        &mut rax
    }

    fn parent_search_l(&mut self, val: ParserToken) -> &mut Option<Rc<RefCell<TreeNode>>> {
        if let Some(left) = &self.left {
            let mut bor = left.borrow_mut();
            if *bor.get_value() == val {
                return &mut Some(Rc::from(RefCell::from(self.clone())));
            } else {
                return bor.search_for_parent_of(val);
            }
        } else {
            return &mut None;
        }
    }

    fn parent_search_r(&mut self, val: ParserToken) -> &mut Option<Rc<RefCell<TreeNode>>> {
        if let Some(right) = &self.right {
            let mut bor = right.borrow_mut();
            if *bor.get_value() == val {
                return &mut Some(Rc::from(RefCell::from(self.clone())));
            } else {
                return bor.search_for_parent_of(val);
            }
        } else {
            return &mut None;
        }
    }

    pub fn vlr_travel(&self, st: &mut String, is_left: bool) { 
        if is_left {
            st.push_str("   LEFT:\n");
        }
        st.push_str(&format!("---Helo: {:?} \n", self.get_value()));
        self.rvlr_travel(st);
        self.lvlr_travel(st);
    }

    fn lvlr_travel(&self, st: &mut String) {
        if let Some(left) = &self.left {
            let bor = left.borrow();
            bor.vlr_travel(st, true);
        } else {
            st.push_str(" Found no nodes on LEFT ");
        }
    }

    fn rvlr_travel(&self, st: &mut String) {
        if let Some(right) = &self.right {
            let bor = right.borrow();
            bor.vlr_travel(st, false);
        } else {
            st.push_str(" Found no nodes on RIGHT ");
        }
    }

    pub fn vlr_print(&self, is_left: bool) {
        if is_left {
            print!("  LEFT:\n");
        }
        print!("---Helo!: {:?} \n", self.get_value());
        self.rvlr_print();
        self.lvlr_print();
    }

    fn lvlr_print(&self) {
        if let Some(left) = &self.left {
            let bor = left.borrow();
            bor.vlr_print(true);
        } else {
            print!(" Found no nodes on LEFT ");
        }

    }

    fn rvlr_print(&self) {
        if let Some(right) = &self.right {
            let bor = right.borrow();
            bor.vlr_print(false);
        } else {
            print!(" Found no nodes on RIGHT ");
        }
    }
}
