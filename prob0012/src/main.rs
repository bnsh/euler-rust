// vim: expandtab shiftwidth=4 tabstop=4:

/*
 * Problem 0012: Highly divisible triangular number
 *
 * The sequence of triangle numbers is generated by adding the natural numbers.
 * So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
 *
 * 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
 *
 * Let us list the factors of the first seven triangle numbers:
 *
 *  1: 1
 *  3: 1,3
 *  6: 1,2,3,6
 * 10: 1,2,5,10
 * 15: 1,3,5,15
 * 21: 1,3,7,21
 * 28: 1,2,4,7,14,28
 * We can see that 28 is the first triangle number to have over five divisors.
 *
 * What is the value of the first triangle number to have over five hundred divisors?
 */

use std::iter::successors;

fn triangles() -> impl Iterator<Item = (u64, u64)> {
    successors(Some((0, 1)), |val| {
        Some(((val.0+val.1), (1+val.1)))
    })
}

fn divisors(val: u64) -> u64 {
    let mut count = 0;
    let mut factor = 1;
    while (factor * factor) <= val {
        if val % factor == 0 {
            if factor * factor != val { count += 2; }
            else { count += 1; }
        }
        factor += 1;
    }
    return count;
}

fn main() {
    for (triangle, _) in triangles() {
        let count = divisors(triangle);
        if count > 500 {
            println!("{}", triangle);
            break;
        }
    }
}
