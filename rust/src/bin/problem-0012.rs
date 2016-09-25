/// Problem 12
/// The sequence of triangle numbers is generated by adding the natural numbers. 
/// So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The 
/// first ten terms would be:
/// 
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
/// 
/// Let us list the factors of the first seven triangle numbers:
/// 
///  1: 1
///  3: 1,3
///  6: 1,2,3,6
/// 10: 1,2,5,10
/// 15: 1,3,5,15
/// 21: 1,3,7,21
/// 28: 1,2,4,7,14,28
/// We can see that 28 is the first triangle number to have over five divisors.
/// 
/// What is the value of the first triangle number to have over 
/// five hundred divisors?
fn main() {
    let mut addend: u64 = 2;
    let mut triangle: u64 = 1;

    loop {
        triangle += addend;
        addend += 1;

        if num_divisors(triangle) > 500 {
            println!("Answer: {}", triangle);
            break
        }
    }
}

fn num_divisors(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }
    let mut num: u64 = 0;
    let mut top: u64 = n;
    let mut i: u64 = 1;
    loop {
        if n % i == 0 {
            num += 2;
            top = n/i;
        }
        i += 1;
        if i >= top {
            break
        }
    }
    num
}
