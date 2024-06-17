// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385.
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 55^2 = 3025.
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is $3025 - 385 = 2640$.
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    let mut sum_squares = 0;
    let mut sum = 0;

    //the equals sign makes it inclusive AKA [1,100]
    for i in 1..=100 {
        sum_squares += i32::pow(i, 2);
        sum += i;
    }

    let square_sum = i32::pow(sum, 2);

    println!("{}", i32::abs(square_sum - sum_squares));
}
