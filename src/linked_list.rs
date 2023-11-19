use std::cmp::Ordering;
use std::collections::{BinaryHeap};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: None
        }
    }

    pub fn print(&self) {
        let mut current = Some(self);
        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = node.next.as_deref();
        }
        println!("None");
    }

    pub fn len(&self) -> i32 {
        let mut l = 0;
        let mut current = Some(self);
        while let Some(node) = current {
            l += 1;
            current = node.next.as_deref();
        }
        l
    }
}

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

struct Solution { }

#[allow(dead_code)]
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut dummy = Box::new(dummy);
        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();

        for _ in 0..n {
            fast = fast.next.unwrap();
        }

        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.next.as_mut().unwrap();
        }

        let next = slow.next.as_mut().unwrap();
        slow.next = next.next.clone();

        dummy.next
    }

    pub fn ll_from_array(arr: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for &a in &arr {
            let node = Box::new(ListNode::new(a));
            *current = Some(node);
            if let Some(ref mut node) = *current {
                current = &mut node.next;
            }
        }

        head
    }

    pub fn is_ll_equal(head1: &Option<Box<ListNode>>, head2: &Option<Box<ListNode>>) -> bool {
        let mut current1 = head1;
        let mut current2 = head2;
        while let (Some(node_1), Some(node_2)) = (current1, current2) {
            if node_1.val != node_2.val {
                return false;
            }
            current1 = &node_1.next;
            current2 = &node_2.next;
        }

        current1.is_none() && current2.is_none()
    }

    pub fn merge_k_lists_fast(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut heap: BinaryHeap<_> = lists.into_iter().flatten().collect();

        let mut head = Box::new(ListNode::new(0));
        let mut current = &mut head;

        while let Some(node) = heap.pop() {
            if let Some(next_node) = node.next {
                heap.push(next_node)
            }
            let new_node = Box::new(ListNode::new(node.val));
            current.next = Some(new_node);
            if let Some(ref mut node) = current.next {
                current = node;
            }
        }
        head.next
    }

    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = None;
        if lists.is_empty() {
            return head;
        }

        let mut current = &mut head;

        let mut smallest_val = i32::MAX;
        let mut si = 0;
        let mut exists = true;

        while exists {
            exists = false;
            for (i, other) in lists.iter().enumerate() {
                let other = other.as_ref();
                if other.is_none() {
                    continue;
                }
                exists = true;
                if other.unwrap().val < smallest_val {
                    smallest_val = other.unwrap().val;
                    si = i;
                }
            }
            if !exists {
                break;
            }
            smallest_val = i32::MAX;

            if let Some(mut node) = lists[si].take() {
                lists[si] = node.next.take();
                *current = Some(node);
                if let Some(ref mut node) = *current {
                    current = &mut node.next;
                }
            }
        }
        head
    }

    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut current = &mut dummy.next;

        while let Some(mut first_node) = current.take() {
            if let Some(second_node) = first_node.next.take() {
                *current = Some(second_node);
                first_node.next = current.as_mut().unwrap().next.take();
                current.as_mut().unwrap().next = Some(first_node);
                current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
            } else {
                *current = Some(first_node);
                break;
            }
        }
        dummy.next
    }
}


#[cfg(test)]
mod tests {
    use crate::linked_list::{Solution};

    #[test]
    fn test_merge_k_list() {
        let arr1 = vec![3, 5, 6, 9];
        let arr2 = vec![8, 90, 95, 101];
        let arr3 = vec![54, 65];

        let head1 = Solution::ll_from_array(arr1);
        let head2 = Solution::ll_from_array(arr2);
        let head3 = Solution::ll_from_array(arr3);

        let vec_heads = vec![head1, head3, head2];

        let merged = Solution::merge_k_lists_fast(vec_heads);

        let expected = Solution::ll_from_array(vec![3, 5, 6, 8, 9, 54, 65, 90, 95, 101]);
        assert!(Solution::is_ll_equal(&merged, &expected));
    }

    #[test]
    fn test_swap_pairs() {
        let arr = vec![1, 2, 3, 4];
        let expected = vec![2, 1, 4, 3];
        let head = Solution::ll_from_array(arr);
        let expected_head = Solution::ll_from_array(expected);

        let result = Solution::swap_pairs(head);

        assert!(Solution::is_ll_equal(&expected_head, &result));
    }
}