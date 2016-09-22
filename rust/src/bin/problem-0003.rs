extern crate rust;
use rust::primes;
/// Problem 3
///
/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
fn main() {
	println!("Problem 3");
	let num: u64 = 600851475143;
	
	let sq: u64 = (num as f64).sqrt().ceil() as u64;
	println!("Computing primes up to {}", sq);
	let prms: Vec<u64> = primes::sieve(sq);
	println!("Done. Checking for factors.");
	
	for n in prms.iter().rev() {
		if num % n == 0 {
			println!("{}", n);
			break;
		}
	}
}


