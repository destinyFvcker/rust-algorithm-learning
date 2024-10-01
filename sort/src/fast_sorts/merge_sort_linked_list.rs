//! leecode 148.排序链表
#[derive(PartialEq, Eq, Clone, Debug)]
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

struct Solution;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        Self::sort_list_range(head)
    }

    fn sort_list_range(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 如果切割到最后是一个孤立节点，则直接返回
        if head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut head = head;
        let mut fast = &head;
        let mut move_count = 1;
        loop {
            if fast.as_ref().unwrap().next.is_none()
                || fast.as_ref().unwrap().next.as_ref().unwrap().next.is_none()
            {
                break;
            }
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            move_count += 1;
        }

        let mut slow = &mut head;
        for _ in 0..move_count {
            slow = &mut slow.as_mut().unwrap().next;
        }

        let mid_node = slow.take();
        let list1 = Self::sort_list_range(head);
        let list2 = Self::sort_list_range(mid_node);
        Self::merge_two_list(list1, list2)
    }

    fn merge_two_list(
        head1: Option<Box<ListNode>>,
        head2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut head1, mut head2) = (head1, head2);
        let mut ans: Option<Box<ListNode>> = None;
        let mut tail = &mut ans;
        while head1.is_some() && head2.is_some() {
            let val1 = head1.as_ref().unwrap().val;
            let val2 = head2.as_ref().unwrap().val;

            let mut target;
            if val1 < val2 {
                target = head1.take();
                head1 = target.as_mut().unwrap().next.take();
            } else {
                target = head2.take();
                head2 = target.as_mut().unwrap().next.take();
            }

            match tail {
                Some(node) => {
                    node.next = target;
                    tail = &mut node.next;
                }
                None => *tail = target,
            }
        }

        while head1.is_some() {
            let mut target = head1.take();
            head1 = target.as_mut().unwrap().next.take();
            // tail.as_mut().unwrap()

            match tail {
                Some(node) => {
                    node.next = target;
                    tail = &mut node.next;
                }
                None => *tail = target,
            }
        }

        while head2.is_some() {
            let mut target = head2.take();
            head2 = target.as_mut().unwrap().next.take();
            // tail.as_mut().unwrap()

            match tail {
                Some(node) => {
                    node.next = target;
                    tail = &mut node.next;
                }
                None => *tail = target,
            }
        }

        return ans;
    }
}
