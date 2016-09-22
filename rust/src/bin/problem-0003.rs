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
	let primes: Vec<u64> = seive_primes(sq);
	println!("Done. Checking for factors.");
	
	for n in primes.iter().rev() {
		if num % n == 0 {
			println!("{}", n);
			break;
		}
	}
}

fn seive_primes(max: u64) -> Vec<u64> {
	let mut seive: Vec<bool> = vec![true; max as usize];
	let top: u64 = (max as f64).sqrt().ceil() as u64;
	let mut incr: u64 = 2;
	let maxs: usize = max as usize;
	loop {
		let mut i:usize = incr as usize;
		loop {
			seive[i] = false;
			i += incr as usize;
			if i >= maxs {
				break;
			}
		}
		incr += 1;
		if incr > top {
			break;
		}
	}
	let mut ans: Vec<u64> = Vec::new();
	for (ind, val) in seive.iter().enumerate() {
		if *val {
			ans.push(ind as u64);
		}
	}
	ans
}
