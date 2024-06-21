// 79.txt contains 50 logins
// each login is 3 numbers from the passcode
// find the shortest possible passcode

/*
My interpretation is that each login requests
3 numbers in _ascending_ order, and the passcode
contains no duplicate numbers.

Example:
[1, 5, 3]
[5, 3, 9]
The shortest passcode from this set is 1539
*/

use std::{collections::HashMap, fs};

fn main() {
    let file_content = fs::read_to_string("src/bin/79.txt").unwrap();
    let mut matrix: Vec<Vec<i32>> = file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut passcode = String::new();

    //loop until matrix is empty
    while matrix.iter().any(|row| row.len() > 0) {
        // find num that only exists at index 0
        let mut not_next_num: HashMap<i32, bool> = HashMap::with_capacity(10);
        for row in matrix.iter() {
            if let Some(first_num) = row.first() {
                not_next_num.entry(*first_num).or_insert(false);
                row.iter().skip(1).for_each(|num| {
                    not_next_num.insert(*num, true);
                });
            }
        }
        let next_num = not_next_num
            .iter()
            .find(|&(_, &value)| value == false)
            .map(|(&key, _)| key)
            .unwrap();

        // remove number from matrix and add to passcode
        matrix
            .iter_mut()
            .for_each(|row| row.retain(|&x| x != next_num));
        passcode.push_str(&next_num.to_string());
    }

    println!("{passcode}");
}
