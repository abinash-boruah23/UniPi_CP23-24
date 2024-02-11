use std::rc::Rc;
use std::cell::RefCell;
use std::i32;

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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mini: i32, maxi: i32) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val >= maxi || node.val <= mini {
                return false;
            }
            return Solution::dfs(node.left.clone(), mini, node.val) && 
                   Solution::dfs(node.right.clone(), node.val, maxi);
        }
        true
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::dfs(root, i32::MIN, i32::MAX)
    }
}

fn main() {
    // Construct the binary tree: [2,1,3]
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    let is_valid = Solution::is_valid_bst(root);
    println!("Is valid BST: {}", is_valid);
}
