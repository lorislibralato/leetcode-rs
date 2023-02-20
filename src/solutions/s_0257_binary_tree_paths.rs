#[allow(unused)]
struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
// #[allow(unused)]
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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        enum Action {
            Root,
            New,
            Same,
        }
        struct Entry {
            action: Action,
            node: Rc<RefCell<TreeNode>>,
            offset: usize,
        }

        let mut paths: Vec<String> = vec![];

        let node = if let Some(node) = root {
            node
        } else {
            return paths;
        };

        let mut remains = Vec::from([Entry {
            action: Action::Root,
            node,
            offset: 0,
        }]);

        while !remains.is_empty() {
            for (mut entry) in remains.drain(0..remains.len()).collect::<Vec<_>>() {
                let node = entry.node.borrow();

                match entry.action {
                    Action::Root => paths.push(node.val.to_string()),
                    Action::New => {
                        paths.push(format!("{}->{}", paths[entry.offset], node.val));
                        entry.offset = paths.len() - 1;
                    }
                    Action::Same => paths[entry.offset] += &format!("->{}", node.val),
                }

                if let Some(left) = &node.left {
                    remains.push(Entry {
                        action: if node.right.is_some() {
                            Action::New
                        } else {
                            Action::Same
                        },
                        node: left.clone(),
                        offset: entry.offset,
                    });
                }
                if let Some(right) = &node.right {
                    remains.push(Entry {
                        action: Action::Same,
                        node: right.clone(),
                        offset: entry.offset,
                    });
                }
            }
        }

        paths
    }
}

#[test]
fn test1() {
    // [1,2,3,null,5]
    let a = TreeNode::new(5);
    let b = TreeNode::new(3);
    let mut c = TreeNode::new(2);
    let mut d = TreeNode::new(1);

    c.right = Some(Rc::new(RefCell::new(a)));
    d.right = Some(Rc::new(RefCell::new(b)));
    d.left = Some(Rc::new(RefCell::new(c)));

    let input = Some(Rc::new(RefCell::new(d)));
    let mut res = Solution::binary_tree_paths(input);
    res.sort();
    let mut output = ["1->2->5", "1->3"];
    output.sort();
    assert_eq!(res, output);
}

#[test]
fn test2() {
    let input = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let res = Solution::binary_tree_paths(input);
    assert_eq!(res, ["1"]);
}
