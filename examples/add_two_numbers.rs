// Definition for singly-linked list.
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

fn main() {
    let mut l = ListNode::new(2);
    let mut l_next = Box::new(ListNode::new(4));
    l_next.next = Some(Box::new(ListNode::new(3)));
    l.next = Some(l_next);

    let mut r = ListNode::new(5);
    let mut r_next = Box::new(ListNode::new(6));
    r_next.next = Some(Box::new(ListNode::new(4)));
    r.next = Some(r_next);

    let sol = Solution::add_two_numbers(Some(Box::new(l)), Some(Box::new(r)));

    let mut expected = Box::new(ListNode::new(7));
    let mut e_next = Box::new(ListNode::new(0));
    e_next.next = Some(Box::new(ListNode::new(8)));
    expected.next = Some(e_next);

    assert_eq!(Some(expected), sol)
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut rem: i32 = 0;
        let mut l: Option<Box<ListNode>> = None;
        let mut l_ref = &mut l;

        let mut l1 = &l1;
        let mut l2 = &l2;

        while let (Some(ln1), Some(ln2)) = (l1, l2) {
            let new_ln = Some(add_listnodes(ln1.val, ln2.val, &mut rem));
            if let Some(ref mut ln) = *l_ref {
                ln.next = new_ln;
                l_ref = &mut ln.next;
            } else {
                *l_ref = new_ln;
            };
            l1 = &ln1.next;
            l2 = &ln2.next;
        }

        if l1.is_none() {
            l1 = l2;
        }

        while let Some(ln1) = l1 {
            let new_ln = Some(add_listnodes(ln1.val, 0, &mut rem));
            if let Some(ref mut ln) = *l_ref {
                ln.next = new_ln;
                l_ref = &mut ln.next;
            } else {
                *l_ref = new_ln;
            };
            l1 = &ln1.next;
        }

        if rem != 0 {
            let rem_node = Box::new(ListNode::new(1));
            if let Some(ref mut ln) = *l_ref {
                ln.next = Some(rem_node); 
            }
        }

        l
    }
}

fn add_listnodes(v1: i32, v2: i32, rem: &mut i32) -> Box<ListNode> {
    let v: i32 = v1 + v2 + *rem;
    *rem = v / 10;
    let v = v % 10;
    Box::new(ListNode::new(v))
}
