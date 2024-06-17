// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder
// What is the smallest positive number that is evenly divisible with no remainder by all of the numbers from 1 to 20?

fn main() {
    let mut num = 2520;
    loop {
        if is_divisible(&num) {
            println!("{}", num);
            return;
        }

        num += 2;
    }
}

fn is_divisible(num: &i32) -> bool {
    for i in 2..20 {
        if num % i != 0 {
            return false;
        }
    }
    true
}
