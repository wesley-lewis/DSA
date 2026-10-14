// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
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

struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Base cases
        if l1.is_none() && l2.is_none() { return None; }
        if l1.is_some() && l2.is_none() { return l1; } 
        if l1.is_none() && l2.is_some() { return l1; } 

        let mut p = l1;
        let mut q = l2;
        let mut l3: Option<Box<ListNode>> = None;
        let mut quotient = 0;
        while p.is_some() && q.is_some() {
            if let Some(node1) = p && let Some(node2) = q {
                let res : i32 = node1.val + node2.val + quotient;
                let rem : i32 = res % 10;
                quotient = res / 10;
                if quotient > 0 {
                    Self::insert(&mut l3, rem);
                }else {
                    Self::insert(&mut l3, res);
                }
                p = node1.next;
                q = node2.next;
            }else {
                unreachable!()
            }
        }
        // Extra remaining nodes from either of them 
        if p.is_some() && q.is_none() {
            while p.is_some() {
                if let Some(node1) = p {
                    let res : i32 = node1.val + quotient;
                    let rem : i32 = res % 10;
                    quotient = res / 10;
                    if quotient > 0 {
                        Self::insert(&mut l3, rem);
                    }else {
                        Self::insert(&mut l3, res);
                    }
                    p = node1.next;
                }
            }
        }
        if q.is_some() && p.is_none() {
            while q.is_some() {
                if let Some(node2) = q {
                    let res : i32 = node2.val + quotient;
                    let rem : i32 = res % 10;
                    quotient = res / 10;
                    if quotient > 0 {
                        Self::insert(&mut l3, rem);
                    }else {
                        Self::insert(&mut l3, res);
                    }
                    q = node2.next;
                }
            }
        }
        if quotient > 0 {
            Self::insert(&mut l3, quotient);
        }
        l3
    }

    fn insert(list: &mut Option<Box<ListNode>>, value: i32) {
        match list {
            None => {
                *list = Some(Box::new(ListNode::new(value)));
            }
            Some(node) => {
                Self::insert(&mut node.next, value);
            }
        }
    }
}

fn main() {

}
