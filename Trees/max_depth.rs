use std::cmp;


#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left = 1 + max_depth(node.left);
            let right = 1 + max_depth(node.right);
            cmp::max(left, right)
        }
        None => 0,
    }
}

fn main() {

    let tree = Some(Box::new(TreeNode {
        value: 1,
        left: Some(Box::new(TreeNode::new(2))),
        right: Some(Box::new(TreeNode {
            value: 3,
            left: Some(Box::new(TreeNode::new(4))),
            right: None,
        })),
    }));

    let depth = max_depth(tree);
    println!("Max Depth: {}", depth);
}
