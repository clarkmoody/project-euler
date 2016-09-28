extern crate rust;
use rust::primes;

/// Problem 27
/// Euler discovered the remarkable quadratic formula:
/// 
/// 		n^2 + n + 41
/// 
/// It turns out that the formula will produce 40 primes for the consecutive 
/// integer values 0 ≤ n ≤ 39. However, when n=40,40^2+40+41=40(40+1)+41 is
/// divisible by 41, and certainly when n=41,41^2+41+41 is clearly divisible 
/// by 41.
/// 
/// The incredible formula n^2 − 79n + 1601 was discovered, which produces 
/// 80 primes for the consecutive values 0 ≤ n ≤ 79. The product of the 
// coefficients, −79 and 1601, is −126479.
/// 
/// Considering quadratics of the form:
/// 
/// 	n^2 + an + b, where |a| < 1000 and |b| ≤ 1000
/// 
/// where |n| is the modulus/absolute value of n
/// e.g. |11| = 11 and |−4| = 4
/// Find the product of the coefficients, a and b, for the quadratic expression 
/// that produces the maximum number of primes for consecutive values of n, 
/// starting with n=0.
fn main() {
	// Get primes up to 1000000
	let sieve: Vec<bool> = primes::sieve_bools(1000000);
	let svals: Vec<u64> = primes::sieve(1000000);
	println!("{} primes under 1000000", svals.len());

	let mut max_consecutive: u16 = 0;
	let mut ab: i32 = 0;

	for a in -999i32..1000i32 {
		for b in &svals {
			if *b > 1000u64 {
				break
			}
			let mut consecutive: u16 = 0;
			for n in 0i32..65535i32 {
				let v: i32 = n*n + a*n + (*b as i32);
				if v >= 0 && sieve[v as usize] {
						consecutive += 1;
				} else {
					break
				}
			}
			if consecutive > max_consecutive {
				max_consecutive = consecutive;
				println!("New consecutive. n={}, a={}, b={}",max_consecutive,a,*b);
				ab = ((a as i64)*(*b as i64)) as i32;
			}
		}
	}
	println!("Answer: {}", ab);
}
