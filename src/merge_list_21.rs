// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {

        let mut head = Box::new(ListNode::new(0));
        let mut current = &mut head;
        
        while let (Some(node1), Some(node2)) = (&mut list1, &mut list2) {
            if node1.val < node2.val {
                let next = node1.next.take();
                current.next = list1.take();
                list1 = next;
            } else {
                let next = node2.next.take();
                current.next = list2.take();
                list2 = next;
            }
            current = current.next.as_mut().unwrap();
        }

        if list1.is_some() {
            current.next = list1;
        }
        if list2.is_some() {
            current.next = list2;
        }

        head.next
    }


    pub fn print_ll(mut list1: Option<Box<ListNode>>) {
        while let Some(node) = list1 {
            print!("{} -> ", node.val);
            list1 = node.next;
        }

        println!("None");
    }
}
