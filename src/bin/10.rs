//find the sum of all primes below 2 million

fn main() {
    let mut sum = 0;

    for i in 2..2000000 {
        if is_prime(i) {
            sum += i;
        }
    }

    println!("{}", sum);
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
