/// 2. 两数相加
/// 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
///
/// 请你将两个数相加，并以相同形式返回一个表示和的链表。
///
/// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
///
/// 示例 1：
/// 输入：l1 = [2,4,3], l2 = [5,6,4]
/// 输出：[7,0,8]
/// 解释：342 + 465 = 807.
///
/// 示例 2：
/// 输入：l1 = [0], l2 = [0]
/// 输出：[0]
///
/// 示例 3：
/// 输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
/// 输出：[8,9,9,9,0,0,0,1]
///
/// 提示：
///
/// 每个链表中的节点数在范围 [1, 100] 内
/// 0 <= Node.val <= 9
/// 题目数据保证列表表示的数字不含前导零

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = None;
    let mut point = &mut res;
    let mut round = (l1, l2, 0, 0); // l1, l2, sum, carry
    loop {
        round = match round {
            (None, None, _, 0) => break,
            (None, None, _, c) => (None, None, c, 0),
            (Some(l), None, _, c) | (None, Some(l), _, c) if l.val + c >= 10 => {
                (l.next, None, l.val + c - 10, 1)
            },
            (Some(l), None, _, c) | (None, Some(l), _, c) => {
                (l.next, None, l.val + c, 0)
            }
            (Some(l1), Some(l2), _, c) if l1.val + l2.val + c >= 10 => {
                (l1.next, l2.next, l1.val + l2.val + c - 10, 1)
            },
            (Some(l1), Some(l2), _, c) => {
                (l1.next, l2.next, l1.val + l2.val + c, 0)
            }
        };
        *point = Some(Box::new(ListNode::new(round.2)));
        point = &mut point.as_mut().unwrap().next;
    }
    res
}

fn main() {
    let mut l1 = Some(Box::new(ListNode::new(4)));
    let mut l2 = Some(Box::new(ListNode::new(6)));
    let mut res = add_two_numbers(l1, l2);
    while res != None {
        println!("{} ", res.as_ref().unwrap().val);
        res = res.unwrap().next;
    }
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}