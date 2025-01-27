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
