// A palindromic number reads the same both ways
// The largest palindrome made from the product of two 2-digit numbers is 9009=91*99
// **Find the largest palindrome made from the product of two 3-digit numbers**
// answer 906609

/* -- Planning --
1. How do we identify a palindrome?
2.

*/

use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let mut largest = 0;
    for a in 900..999 {
        for b in 900..a {
            let num = a * b;
            if is_palindrome(&num) && num > largest {
                largest = num;
            }
        }
    }

    println!("{}", largest);
    println!("took {:.2?}", start_time.elapsed());
}

fn is_palindrome(num: &i32) -> bool {
    let str = num.to_string();
    let len = str.len();
    for i in 0..(len / 2) {
        let left = str.chars().nth(i).unwrap();
        let right = str.chars().nth(len - 1 - i).unwrap();
        if left != right {
            return false;
        }
    }
    true
}
