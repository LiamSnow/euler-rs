// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13
// What is the 10\,001st prime number?

fn main() {
    let mut count = 0;
    let mut i = 2;

    loop {
        if is_prime(i) {
            count += 1;
        }

        if count == 10001 {
            println!("10,001st prime number is {}", i);
            return;
        }

        i += 1;
    }
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
