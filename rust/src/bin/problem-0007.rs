extern crate rust;
use rust::primes;

/// Problem 7
///
/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see 
/// that the 6th prime is 13.
/// 
/// What is the 10 001st prime number?
fn main() {
	println!("Problem 7");
	let nth: u64 = 6;

	// Estimate for prime density is x/ln(x)

	println!("Answer: {}", nth);
}
