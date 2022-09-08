#[allow(unused)]
struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(unused)]
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

use std::cell::RefCell;
use std::rc::Rc;
#[allow(unused)]
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            Self::check_node(root, None, None)
        } else {
            false
        }
    }

    fn check_node(node: Rc<RefCell<TreeNode>>, min: Option<i32>, max: Option<i32>) -> bool {
        let node = node.borrow();
        let left_ok = if let Some(left) = &node.left {
            let val = left.borrow().val;
            let range_ok = if let Some(min) = min {
                min < val
            } else {
                true
            } && 
            if let Some(max) = max {
                max > val
            } else {
                true
            };
            val < node.val && range_ok && Self::check_node(Rc::clone(left), min, Some(node.val))
        } else {
            true
        };

        let right_ok = if let Some(right) = &node.right {
            let val = right.borrow().val;
            let range_ok = if let Some(min) = min {
                min < val
            } else {
                true
            } && 
            if let Some(max) = max {
                max > val
            } else {
                true
            };
            val > node.val && range_ok && Self::check_node(Rc::clone(right), Some(node.val), max)
        } else {
            true
        };

        left_ok && right_ok
    }
}

#[test]
fn test1() {
    let root = TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    };

    assert!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))));
}

#[test]
fn test2() {
    let root = TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    };

    assert!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))));
}

#[test]
fn test3() {
    let root = TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    };

    assert!(!Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))))
}

#[test]
fn test4() {
    let root = TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    };

    assert!(!Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))))
}


#[test]
fn test5() {
    assert_eq!((-2147483648, 2147483647), (i32::MIN, i32::MAX));
    
    let root = TreeNode {
        val: i32::MIN,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(i32::MAX)))),
    };

    assert!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))));
}
