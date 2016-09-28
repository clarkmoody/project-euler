/// Problem 23
/// A perfect number is a number for which the sum of its proper divisors is 
/// exactly equal to the number. For example, the sum of the proper divisors of
/// 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number
/// 
/// A number n is called deficient if the sum of its proper divisors is less 
/// than n and it is called abundant if this sum exceeds n.
/// 
/// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest 
/// number that can be written as the sum of two abundant numbers is 24. By 
/// mathematical analysis, it can be shown that all integers greater than 28123 
/// can be written as the sum of two abundant numbers. However, this upper limit 
/// cannot be reduced any further by analysis even though it is known that the 
/// greatest number that cannot be expressed as the sum of two abundant numbers 
/// is less than this limit.
/// 
/// Find the sum of all the positive integers which cannot be written as the 
/// sum of two abundant numbers.
fn main() {
    let max: u64 = 28123;
    let mut abundant: Vec<u64> = Vec::new();
    for i in 1u64..(max+1) {
    // for i in 1u64..100u64 {
        if sum_divisors(i) > i {
            abundant.push(i);
        }
    }
    // println!("Found {} abundant numbers less than {}", abundant.len(), max);
    // Pre-compute all abundant pair sums
    let mut sums: Vec<bool> = vec![false; max as usize+1];
    for j in 0..abundant.len() {
        for k in 0..(j+1) {
            let s = (abundant[j] + abundant[k]) as usize;
            if s >= sums.len() {
                break
            }
            sums[s] = true;
        }
    }

    let mut sum: u64 = 0;
    for i in 0..sums.len() {
        if !sums[i] {
            sum += i as u64;
        }
    }
    
    println!("Answer: {}", sum);
}

fn sum_divisors(n: u64) -> u64 {
    if n <= 2 {
        return 1;
    }
    let mut sum: u64 = 1;
    let mut top: u64 = n;
    let mut i: u64 = 2;
    loop {
        if n % i == 0 {
            sum += i;
            if i != n/i {
                sum += n/i;
            }
            top = n/i;
        }
        i += 1;
        if i >= top {
            break
        }
    }
    sum
}
