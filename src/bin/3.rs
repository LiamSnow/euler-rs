// The prime factors of 13195 are 5, 7, 13 and 29
// find largest prime factor of 600851475143
// answer 6857

use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let num: i64 = 600851475143;
    let end = (num as f64).sqrt() as i64;

    //iter is faster than loop!
    //https://doc.rust-lang.org/book/ch13-04-performance.html
    let prime_factor_res = (2..=end).rev().find(|&i| num % i == 0 && is_prime(i));

    match prime_factor_res {
        Some(prime_factor) => println!("{}", prime_factor),
        None => println!("No prime factor found."),
    }

    println!("took {:.2?}", start_time.elapsed());
}

fn is_prime(num: i64) -> bool {
    if num <= 1 {
        return false;
    }
    let end = (num as f64).sqrt() as i64;
    for i in 2..=end {
        if num % i == 0 {
            return false;
        }
    }
    true
}
