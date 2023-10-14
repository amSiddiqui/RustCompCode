#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self {
            val,
            next: None
        }
    }

    fn print(&self) {
        let mut current = Some(self);
        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = node.next.as_deref();
        }
        println!("None");
    }
}

