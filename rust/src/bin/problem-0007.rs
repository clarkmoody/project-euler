extern crate rust;
use rust::{primes, numeric};

/// Problem 7
///
/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see 
/// that the 6th prime is 13.
/// 
/// What is the 10001st prime number?
fn main() {
	println!("Problem 7");
	let nth: u64 = 10001;

	// Estimate for prime density is n/ln(n) = pi(n)
    // (there are pi primes less than n)
    let pi = &|x: f64| x/x.ln() - nth as f64;
    let est_size: u64 = numeric::fsolve(pi, nth as f64) as u64;

    let prms: Vec<u64> = primes::sieve(est_size);

    if prms.len() < nth as usize {
        println!("Need to sieve more primes. Got {}", prms.len());
    } else {
        println!("Answer: {}", prms[nth as usize]);
    }
}
