use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    let mut root: Option<Rc<RefCell<TreeNode>>> = None;

    insert_iter(&mut root, 3);
    insert_iter(&mut root, 5);
    insert_iter(&mut root, 4);

    //println!("{:#?}", root);

    println!("{:?}", inorder_traversal(root));
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

pub fn insert_iter(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
    if root.is_none() {
        *root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        return;
    }

    let mut current = root.clone();

    while let Some(node) = current {
        let mut n = node.borrow_mut();

        if val < n.val {
            if n.left.is_none() {
                n.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                return;
            }
            current = n.left.clone();
        } else {
            if n.right.is_none() {
                n.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                return;
            }
            current = n.right.clone();
        }
    }
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut current = root;

    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            current = node.borrow().left.clone();
            stack.push(node);
        }

        let node = stack.pop().unwrap();
        result.push(node.borrow().val);
        current = node.borrow().right.clone();
    }

    result
}
