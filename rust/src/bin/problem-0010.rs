extern crate rust;
use rust::primes;

/// Problem 10
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
/// 
/// Find the sum of all the primes below two million.
fn main() {
    let p: Vec<u64> = primes::sieve(2000000u64);
    let s: u64 = p.iter().sum();
    println!("Answer: {}", s);
}
