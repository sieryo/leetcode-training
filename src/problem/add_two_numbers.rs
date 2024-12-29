/*

---- KESALAHAN PEMULA ----

    Disini saya melakukan beberapa kesalahan pada fungsi slow_naive (yang saya pikirkan sendiri solusinya), yang dimana
    saya mengubah dulu angka problemnya menjadi char, yang hanya boros-boros CPU doang. Dimana akhirnya saya nanya ke chatGPT
    dan memahami solusi saya mirip konsepnya, namun lebih tidak "to the point" dan panjang banget buset line nya.

--------------------------


*/

use crate::data_type::list_node::ListNode;

pub fn add_two_numbers_slow_naive(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut vec_l1: Vec<char> = Vec::new();
    let mut vec_l2: Vec<char> = Vec::new();
    let mut current_l1 = &l1;
    let mut current_l2 = &l2;

    while let Some(next) = &current_l1 {
        vec_l1.push(char::from_digit(next.val as u32, 10).unwrap());
        current_l1 = &next.next;
    }

    while let Some(next) = &current_l2 {
        vec_l2.push(char::from_digit(next.val as u32, 10).unwrap());
        current_l2 = &next.next;
    }
    let flag: bool = vec_l1.len() >= vec_l2.len();
    let mut remaining: Option<u32> = None;

    // let new_vec_l1 = vec_l1.iter().rev().enumerate();
    // let new_vec_l2 = vec_l2.iter().rev().enumerate();

    let mut result: Vec<char> = Vec::new();
    if flag {
        for (i, c) in vec_l1.iter().enumerate() {
            if let Some(c_l2) = vec_l2.get(i) {
                if let Some(re) = remaining {
                    let r = c.to_digit(10).unwrap() + c_l2.to_digit(10).unwrap() + re;

                    let count = r / 10;
                    let remainder = r % 10;

                    remaining = Some(count);

                    result.push(char::from_digit(remainder, 10).unwrap());
                } else {
                    let r = c.to_digit(10).unwrap() + c_l2.to_digit(10).unwrap();

                    let count = r / 10;
                    let remainder = r % 10;

                    remaining = Some(count);

                    result.push(char::from_digit(remainder, 10).unwrap());
                }
            } else {
                if let Some(re) = remaining {
                    let r = c.to_digit(10).unwrap() + re;

                    let count = r / 10;
                    let remainder = r % 10;

                    remaining = Some(count);

                    result.push(char::from_digit(remainder, 10).unwrap());
                } else {
                    let r = c.to_digit(10).unwrap();

                    let count = r / 10;
                    let remainder = r % 10;

                    remaining = Some(count);

                    result.push(char::from_digit(remainder, 10).unwrap());
                }
            }
        }
        let c = remaining.unwrap();

        if c >= 1 {
            result.push(char::from_digit(c, 10).unwrap());
        }
    } else {
        for (i, c) in vec_l2.iter().enumerate() {
            if let Some(c_l1) = vec_l1.get(i) {
                if let Some(re) = remaining {
                    let r = c.to_digit(10).unwrap() + c_l1.to_digit(10).unwrap() + re;

                    let count = r / 10;
                    let remainder = r % 10;

                    remaining = Some(count);

                    result.push(char::from_digit(remainder, 10).unwrap());
                } else {
                    let r = c.to_digit(10).unwrap() + c_l1.to_digit(10).unwrap();

                    let count = r / 10;
                    let remainder = r % 10;

                    remaining = Some(count);

                    result.push(char::from_digit(remainder, 10).unwrap());
                }
            } else {
                if let Some(re) = remaining {
                    let r = c.to_digit(10).unwrap() + re;

                    let count = r / 10;
                    let remainder = r % 10;

                    remaining = Some(count);

                    result.push(char::from_digit(remainder, 10).unwrap());
                } else {
                    let r = c.to_digit(10).unwrap();

                    let count = r / 10;
                    let remainder = r % 10;

                    remaining = Some(count);

                    result.push(char::from_digit(remainder, 10).unwrap());
                }
            }
        }
        let c = remaining.unwrap();

        if c >= 1 {
            result.push(char::from_digit(c, 10).unwrap());
        }
    }

    dbg!(result.clone());

    let first_char = result[0].to_digit(10).unwrap();

    let mut list = Some(Box::new(ListNode::new(first_char as i32)));
    let mut curr = &mut list;

    for (i, c) in result.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let cur = c.to_digit(10).unwrap(); 
        let new_node = Some(Box::new(ListNode::new(cur as i32)));

        if let Some(ref mut current_node) = curr {
            current_node.next = new_node;
            curr = &mut current_node.next;
        }
    }

    list
}

// Dari chatgpt anjay, gg banget gini ternyata
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode::new(0)));
    let mut p = l1;
    let mut q = l2;
    let mut current = &mut dummy_head;
    let mut carry = 0;

    while p.is_some() || q.is_some() {
        let x = p.as_ref().map_or(0, |node| node.val);
        let y = q.as_ref().map_or(0, |node| node.val);
        let sum = carry + x + y;
        carry = sum / 10;

        if let Some(ref mut curr_node) = current {
            curr_node.next = Some(Box::new(ListNode::new(sum % 10)));
            current = &mut curr_node.next;
        }

        if let Some(node) = p {
            p = node.next;
        }
        if let Some(node) = q {
            q = node.next;
        }
    }

    if carry > 0 {
        if let Some(ref mut curr_node) = current {
            curr_node.next = Some(Box::new(ListNode::new(carry)));
        }
    }

    dummy_head.unwrap().next
}