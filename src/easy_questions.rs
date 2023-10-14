// Definition for singly-linked list.

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
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
}


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
            right: None,
        }
    }
}


struct CacheKey {
    ptr: *const RefCell<TreeNode>,
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl Eq for CacheKey {}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state)
    }
}


struct Memoiser {
    cache: HashMap<(CacheKey, u32), u32>,
}

impl Memoiser {
    fn new() -> Self {
        Self {
            cache: HashMap::new()
        }
    }


    fn traverse(&mut self, root: Option<Rc<RefCell<TreeNode>>>, depth: u32) -> u32 {
        if let Some(rc) = &root {
            let key = CacheKey { ptr: Rc::as_ptr(rc) };
            if let Some(&cached_value) = self.cache.get(&(key, depth)) {
                return cached_value;
            }
        }
        let mut result = depth;
        if let Some(node) = &root {
            result = max(self.traverse(node.borrow().left.clone(), depth + 1),
                         self.traverse(node.borrow().right.clone(), depth + 1));
        }

        if let Some(rc) = &root {
            let key = CacheKey { ptr: Rc::as_ptr(rc) };
            self.cache.insert((key, depth), result);
        }

        result
    }
}


pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = vec![];
        let mut remainder = false;
        let (mut a_i, mut b_i) =
            (a.chars().rev().peekable(), b.chars().rev().peekable());
        let zip_iter = if a.len() < b.len() {
            a_i.by_ref().zip(b_i.by_ref())
        } else {
            b_i.by_ref().zip(a_i.by_ref())
        };
        for (i, j) in zip_iter {
            match (i == '1', j == '1') {
                (false, false) => {
                    if remainder {
                        result.push('1');
                    } else {
                        result.push('0');
                    }
                    remainder = false;
                }
                (true, true) => {
                    if remainder {
                        result.push('1');
                    } else {
                        result.push('0');
                    }
                    remainder = true;
                }
                _ => {
                    if remainder {
                        result.push('0');
                    } else {
                        result.push('1');
                    }
                }
            }
        }

        let remaining = if a_i.peek().is_some() { a_i } else { b_i };

        for i in remaining {
            if remainder {
                if i == '1' {
                    result.push('0');
                } else {
                    result.push('1');
                    remainder = false;
                }
            } else {
                result.push(i);
            }
        }

        if remainder {
            result.push('1');
        }

        result.iter().rev().collect()
    }

    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while let Some(node) = current {
            let next = &mut node.next;
            while let Some(next_node) = next {
                if next_node.val == node.val {
                    *next = next_node.next.take();
                } else {
                    break;
                }
            }
            current = next;
        }

        head
    }

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in m as usize..(m + n) as usize {
            nums1[i] = nums2[i - m as usize];
        }
        nums1.sort();
    }

    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => {
                true
            }
            (Some(np), Some(nq)) => {
                let node_p = np.borrow();
                let node_q = nq.borrow();
                node_p.val == node_q.val &&
                    Solution::is_same_tree(node_p.left.clone(), node_q.left.clone()) &&
                    Solution::is_same_tree(node_p.right.clone(), node_q.right.clone())
            }
            _ => {
                false
            }
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                Solution::left_right_equal(node.borrow().left.clone(),
                                           node.borrow().right.clone())
            }
            None => {
                true
            }
        }
    }

    fn left_right_equal(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (Some(n1), Some(n2)) => {
                let node_1 = n1.borrow();
                let node_2 = n2.borrow();
                node_1.val == node_2.val &&
                    Solution::left_right_equal(node_1.left.clone(),
                                               node_2.right.clone()) &&
                    Solution::left_right_equal(node_1.right.clone(),
                                               node_2.left.clone())
            }
            (None, None) => {
                true
            }
            _ => {
                false
            }
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::traverse_max(root, 0) as i32
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                Solution::traverse_min(node, 0) as i32
            }
            None => {
                0
            }
        }
    }

    fn traverse_max(child: Option<Rc<RefCell<TreeNode>>>, depth: u32) -> u32 {
        match child {
            Some(node) => {
                max(Solution::traverse_max(node.borrow().left.clone(), depth + 1),
                    Solution::traverse_max(node.borrow().right.clone(), depth + 1))
            }
            None => {
                depth
            }
        }
    }

    fn traverse_min(child: Rc<RefCell<TreeNode>>, depth: u32) -> u32 {
        match (child.borrow().left.clone(), child.borrow().right.clone()) {
            (Some(node_l), Some(node_r)) => {
                min(Solution::traverse_min(node_l, depth + 1),
                    Solution::traverse_min(node_r, depth + 1))
            }
            (Some(node_l), None) => {
                Solution::traverse_min(node_l, depth + 1)
            }
            (None, Some(node_r)) => {
                Solution::traverse_min(node_r, depth + 1)
            }
            _ => {
                depth + 1
            }
        }
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        Solution::traverse_sum(root, 0, target_sum)
    }

    fn traverse_sum(node: Option<Rc<RefCell<TreeNode>>>, sum: i32, target_sum: i32) -> bool {
        match node {
            Some(rc) => {
                let rc_b = rc.borrow();
                let acc = rc_b.val + sum;
                acc == target_sum ||
                    Solution::traverse_sum(rc_b.left.clone(), acc, target_sum) ||
                    Solution::traverse_sum(rc_b.right.clone(), acc, target_sum)
            },
            None => {
                sum == target_sum
            }
        }
    }


    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut memo = Memoiser::new();
        Solution::is_balanced_memo(root, &mut memo)
    }

    fn is_balanced_memo(root: Option<Rc<RefCell<TreeNode>>>, memo: &mut Memoiser) -> bool {
        match root {
            Some(rc) => {
                let left_d = memo.traverse(rc.borrow().left.clone(), 1);
                let right_d = memo.traverse(rc.borrow().right.clone(), 1);
                let is_root_balanced = match left_d.checked_sub(right_d) {
                    Some(diff) => {
                        diff < 2
                    }
                    None => {
                        right_d - left_d < 2
                    }
                };
                is_root_balanced &&
                    Solution::is_balanced_memo(rc.borrow().left.clone(), memo) &&
                    Solution::is_balanced_memo(rc.borrow().right.clone(), memo)
            }
            None => {
                true
            }
        }
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::create_node(nums.as_ref(), 0, nums.len() - 1)
    }

    fn create_node(nums: &Vec<i32>, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r {
            None
        } else {
            let mid = (l + r) / 2;
            let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            if mid > 0 {
                root.borrow_mut().left = Solution::create_node(nums, l, mid - 1);
            }
            root.borrow_mut().right = Solution::create_node(nums, mid + 1, r);
            Some(root)
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::easy_questions::{Solution, TreeNode, Rc, RefCell};

    #[test]
    fn test_add_binary() {
        assert_eq!("100", Solution::add_binary("11".to_string(), "1".to_string()));
        assert_eq!("10101", Solution::add_binary("1010".to_string(), "1011".to_string()));
    }

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];

        Solution::merge(nums1.as_mut(), 3, nums2.as_mut(), 3);

        println!("{:?}", nums1);
    }

    #[test]
    fn test_balanced_tree() {
        let root = Rc::new(RefCell::new(TreeNode {
            val: 1,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
            }))),
        }));

        assert!(!Solution::is_balanced(Some(root)));
    }
}
