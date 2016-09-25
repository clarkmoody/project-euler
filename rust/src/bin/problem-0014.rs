/// Problem 14
/// The following iterative sequence is defined for the set of positive integers:
/// 
/// n → n/2 (n is even)
/// n → 3n + 1 (n is odd)
/// 
/// Using the rule above and starting with 13, we generate the following sequence:
/// 
/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
/// It can be seen that this sequence (starting at 13 and finishing at 1) contains 
/// 10 terms. Although it has not been proved yet (Collatz Problem), it is thought 
/// that all starting numbers finish at 1.
/// 
/// Which starting number, under one million, produces the longest chain?
/// 
/// NOTE: Once the chain starts the terms are allowed to go above one million.
fn main() {
    let top: u64 = 1000000;
    let mut max: u64 = 0;
    let mut num_max: u64 = 0;

    for i in 2..top {
        let c = collatz(i);
        if c > max {
            max = c;
            num_max = i;
            println!("{} -> {}", i, c);
        }
    }

    println!("Answer: {}", num_max);
}

fn collatz(n: u64) -> u64 {
    let mut count = 1;
    let mut num: u64 = n;
    loop {
        if num == 1 {
            break
        }
        count += 1;
        num = match num % 2 {
            0 => num/2,
            1 => 3*num+1,
            _ => 1
        };
    }
    count
}
