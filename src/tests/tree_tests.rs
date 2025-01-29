#[cfg(test)]
pub mod tree_tests {
    use crate::tree::TreeNode;
    use crate::parser::{ParserToken, ParserTokenType};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn it_works() {
        let tree = create_tree1();
        assert_eq!(tree.borrow_mut().get_left().unwrap().borrow_mut().get_value().get_type(), ParserTokenType::Func);
        assert_eq!(tree.borrow_mut().get_right().unwrap().borrow_mut().get_value().get_type(), ParserTokenType::FuncHeader);
        assert_eq!(tree.borrow_mut().get_right().unwrap().borrow_mut().get_left().unwrap().borrow_mut().get_value().get_type(), ParserTokenType::Op);
    }   

    fn create_tree1() -> Rc<RefCell<TreeNode>> {
        let mut head = TreeNode::new(ParserToken::new(ParserTokenType::Id, String::from("test")));

        let mut head_l = TreeNode::new(ParserToken::new(ParserTokenType::Func, String::from("func")));
        let mut head_r = TreeNode::new(ParserToken::new(ParserTokenType::FuncHeader, String::from("func a1")));

        head.set_left(Rc::from(RefCell::from(head_l)));
        let head_r_rc = Rc::from(RefCell::from(head_r));
        head.set_right(head_r_rc.clone());

        let mut head_r_l = TreeNode::new(ParserToken::new(ParserTokenType::Op, String::from("+")));
        head_r_rc.borrow_mut().set_left(Rc::from(RefCell::from(head_r_l)));

        Rc::from(RefCell::from(head))
    }
}
