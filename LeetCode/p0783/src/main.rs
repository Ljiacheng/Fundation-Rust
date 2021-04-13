/// 783. 二叉搜索树节点最小距离
///
/// 给你一个二叉搜索树的根节点 root ，返回树中任意两不同节点值之间的最小差值 。
///
/// 注意：本题与 530：https://leetcode-cn.com/problems/minimum-absolute-difference-in-bst/ 相同
///
/// 示例 1：
///
/// 输入：root = [4,2,6,1,3]
/// 输出：1
///
/// 示例 2：
/// 输入：root = [1,0,48,null,null,12,49]
/// 输出：1
///
/// 提示：
///
/// 树中节点数目在范围 [2, 100] 内
/// 0 <= Node.val <= 105

use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::Borrow;

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

pub fn min_diff_in_bst_m(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32, pre: &mut i32) {
    if let Some(r) = root {
        min_diff_in_bst_m(r.borrow().left.clone(), res, pre);
        if *pre >= 0 {
            *res = (*res).min(r.borrow().val - *pre);
        }
        *pre = r.borrow().val;
        min_diff_in_bst_m(r.borrow().right.clone(), res, pre);
    }
}

pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = i32::max_value();
    let mut pre = -1;
    min_diff_in_bst_m(root, &mut res, &mut pre);
    res
}

fn main() {
    println!("Hello, world!");
}
