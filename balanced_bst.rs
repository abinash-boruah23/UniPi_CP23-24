use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
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
            right: None
        }
    }
}

struct Solution;

impl Solution {
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                1 + Self::dfs(node.left.as_ref()).max(Self::dfs(node.right.as_ref()))
            },
            None => 0,
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let left_height = Self::dfs(node.left.as_ref());
                let right_height = Self::dfs(node.right.as_ref());
                (left_height - right_height).abs() <= 1 && 
                    Self::is_balanced(node.left.clone()) && 
                    Self::is_balanced(node.right.clone())
            },
            None => true,
        }
    }
}

fn main() {
    // Example usage
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));

    println!("{}", Solution::is_balanced(root));
}
