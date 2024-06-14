// find sum of even-values in Fibonacci sequence (under 4 million)
// answer 4613732

fn main() {
    let (mut first_term, mut second_term) = (1, 2);
    let mut sum = second_term;
    loop {
        let next_term = first_term + second_term;
        if next_term >= 4000000 {
            break;
        }
        if next_term % 2 == 0 {
            sum += next_term;
        }
        first_term = second_term;
        second_term = next_term;
    }
    println!("{}", sum);
}
