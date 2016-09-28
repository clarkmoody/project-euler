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
    println!("{}->{}", 1, sum_divisors(1));
    println!("{}->{}", 2, sum_divisors(2));
    println!("{}->{}", 5, sum_divisors(5));
    println!("{}->{}", 12, sum_divisors(12));
    println!("{}->{}", 28, sum_divisors(28));
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
            sum += n/i;
            top = n/i;
        }
        i += 1;
        if i >= top {
            break
        }
    }
    sum
}
