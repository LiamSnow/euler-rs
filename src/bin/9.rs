//a + b + c = 1000
//a^2 + b^2 = c^2
//find abc

const SUM: i32 = 1000;

fn main() {
    for a in 1..(SUM/2) {
        for b in a..(SUM/2) {
            let c = 1000 - a - b;
            if i32::pow(a, 2) + i32::pow(b, 2) == i32::pow(c, 2) {
                println!("{}", a * b * c);
                return;
            }
        }
    }
}

