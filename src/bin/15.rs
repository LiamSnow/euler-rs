/*
2x2 -> 4 moves -> 16 routes -> 6 valid routes

0 = right, 1 = down
Must have grid_size 0's and 1's

-> We are finding # of balanced
   binary number in grid_size * 2

Resources Used:
https://math.libretexts.org/Bookshelves/Combinatorics_and_Discrete_Mathematics/Applied_Discrete_Structures_(Doerr_and_Levasseur)/02%3A_Combinatorics/2.04%3A_Combinations_and_the_Binomial_Theorem
*/

fn main() {
    let grid_size = 20;
    let num_bits = grid_size * 2; //# moves
    let num_routes = combination(num_bits, grid_size);

    println!("{}", num_routes);
}

//good ol' combination formula 😎 #1
fn combination(n: i64, r: i64) -> i64 {
    (0..r).fold(1, |acc, i| acc * (n-i) / (i+1))
}


