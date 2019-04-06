// 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。

// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。

// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

// 示例：

// 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
// 输出：7 -> 0 -> 8
// 原因：342 + 465 = 807

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1: Option<Box<ListNode>> = l1;
        let mut l2: Option<Box<ListNode>> = l2;

        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut current: Option<&mut Box<ListNode>> = head.as_mut();
        let mut result: i32 = 0;

        while l1.is_some() || l2.is_some() {
            match l1 {
                Some(l1_point) => {
                    result += l1_point.val;
                    l1 = l1_point.next;
                }
                None => {}
            }

            match l2 {
                Some(l2_point) => {
                    result += l2_point.val;
                    l2 = l2_point.next;
                }
                None => {}
            }

            match current {
                Some(mut current_point) => {
                    let next_node: Box<ListNode> = Box::new(ListNode::new(result % 10));
                    current_point.next = Some(next_node);
                    current = current_point.next.as_mut();
                    result /= 10;
                }
                None => {}
            }
        }

        while result > 0 {
            match current {
                Some(mut current_point) => {
                    let next_node: Box<ListNode> = Box::new(ListNode::new(result % 10));
                    current_point.next = Some(next_node);
                    current = current_point.next.as_mut();
                    result /= 10;
                }
                None => {}
            }
        }

        head.and_then(|item| item.next)
    }
}

#[test]
fn add_two_numbers() {
    let list1: Option<Box<ListNode>> = Some(Box::new(ListNode {
        next: Some(Box::new(ListNode {
            next: Some(Box::new(ListNode { next: None, val: 3 })),
            val: 4,
        })),
        val: 2,
    }));

    let list2: Option<Box<ListNode>> = Some(Box::new(ListNode {
        next: Some(Box::new(ListNode {
            next: Some(Box::new(ListNode { next: None, val: 4 })),
            val: 6,
        })),
        val: 5,
    }));

    let list3: Option<Box<ListNode>> = Some(Box::new(ListNode {
        next: Some(Box::new(ListNode {
            next: Some(Box::new(ListNode { next: None, val: 8 })),
            val: 0,
        })),
        val: 7,
    }));

    assert_eq!(Solution::add_two_numbers(list1, list2), list3);

    assert_eq!(
        Solution::add_two_numbers(
            Some(Box::new(ListNode::new(5))),
            Some(Box::new(ListNode::new(5)))
        ),
        Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode::new(1)))
        }))
    );
}
