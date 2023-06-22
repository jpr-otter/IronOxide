use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn display_tree(f: &mut fmt::Formatter, tree: &Tree, prefix: String, is_left: bool) -> fmt::Result {
            if let Some(node) = tree {
                let node = node.borrow();
                let new_prefix = format!("{}{}{}", prefix, if is_left { "│   " } else { "    " }, "");
                display_tree(f, &node.right, new_prefix.clone(), false)?;
                writeln!(f, "{}{}───{}", prefix, if is_left { "└" } else { "┌" }, node.val)?;
                display_tree(f, &node.left, new_prefix.clone(), true)?;
            }
            Ok(())
        }
        display_tree(f, &self.right, "".to_string(), false)?;
        write!(f, "{}", self.val)?;
        display_tree(f, &self.left, "".to_string(), true)?;
        Ok(())
    }
}

pub fn invert_tree(root: Tree) -> Tree {
    if let Some(node) = root.clone() {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        node.borrow_mut().left = invert_tree(right);
        node.borrow_mut().right = invert_tree(left);
    }
    root
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let node2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let node7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    let node1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let node3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let node9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));

    root.as_ref().unwrap().borrow_mut().left = node2.clone();
    root.as_ref().unwrap().borrow_mut().right = node7.clone();
    node2.as_ref().unwrap().borrow_mut().left = node1;
    node2.as_ref().unwrap().borrow_mut().right = node3;
    node7.as_ref().unwrap().borrow_mut().left = node6;
    node7.as_ref().unwrap().borrow_mut().right = node9;

    println!("Original tree:");
    println!("{}", root.as_ref().unwrap().borrow());
    let inverted_root = invert_tree(root);
    println!("Inverted tree:");
    println!("{}", inverted_root.as_ref().unwrap().borrow());
}
