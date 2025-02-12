mod q_2_6;

use q_2_6::{is_palindrome, ListNode};

fn main() {
// 回文リストの作成例：1 -> 2 -> 3 -> 2 -> 1
let mut n1 = Box::new(ListNode::new(1));
let mut n2 = Box::new(ListNode::new(2));
let mut n3 = Box::new(ListNode::new(3));
let mut n4 = Box::new(ListNode::new(2));
let n5 = Box::new(ListNode::new(1));

n4.next = Some(n5);
n3.next = Some(n4);
n2.next = Some(n3);
n1.next = Some(n2);
let palindrome_list = Some(n1);

// 回文でないリストの作成例：1 -> 2 -> 3 -> 4
let mut a1 = Box::new(ListNode::new(1));
let mut a2 = Box::new(ListNode::new(2));
let mut a3 = Box::new(ListNode::new(3));
let a4 = Box::new(ListNode::new(4));

a3.next = Some(a4);
a2.next = Some(a3);
a1.next = Some(a2);
let non_palindrome_list = Some(a1);

println!("palindrome_list is palindrome? {}", is_palindrome(&palindrome_list));
println!("non_palindrome_list is palindrome? {}", is_palindrome(&non_palindrome_list));
}
/*
時間計算量: リストを2回走査するので O(n)
空間計算量: 前半部分の値をスタックに保持するため O(n)
*/