use std::cmp;

// Define a simple TreeNode structure
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    fn helper(root: Option<Box<TreeNode>>, maxi: &mut i32) -> i32 {
        if let Some(node) = root {
            let left = cmp::max(0, Solution::helper(node.left, maxi));
            let right = cmp::max(0, Solution::helper(node.right, maxi));
            *maxi = cmp::max(*maxi, node.val + left + right);
            return cmp::max(left, right) + node.val;
        }
        0
    }

    pub fn max_path_sum(root: Option<Box<TreeNode>>) -> i32 {
        let mut maxi = i32::MIN;
        Solution::helper(root, &mut maxi);
        maxi
    }
}

fn main() {
    // Create the binary tree: [-10, 9, 20, null, null, 15, 7]
    let tree = Some(Box::new(TreeNode {
        val: -10,
        left: Some(Box::new(TreeNode::new(9))),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode::new(15))),
            right: Some(Box::new(TreeNode::new(7))),
        })),
    }));

    let result = Solution::max_path_sum(tree);
    println!("Maximum Path Sum: {}", result);
}
