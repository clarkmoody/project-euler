/// Problem 24
/// A permutation is an ordered arrangement of objects. For example, 3124 is one 
/// possible permutation of the digits 1, 2, 3 and 4. If all of the permutations 
/// are listed numerically or alphabetically, we call it lexicographic order. 
/// The lexicographic permutations of 0, 1 and 2 are:
///
///			012   021   102   120   201   210
///
/// What is the millionth lexicographic permutation of the digits 
/// 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
fn main() {
	let p: Vec<u64> = nth_permutation(10, 1000000);
	print!("Answer: ");
	for d in p {
		print!("{}", d);
	}
	print!("\n");
}

/// Compute the nth permutation of a vector of length l. Returns a vector of 
/// array indices corresponding to the premutation.
///
/// asserteq!(nth_permutation(3,1), [0,1,2]);
///
/// Permutations are numbered from 1..n!
fn nth_permutation(l: u64, n: u64) -> Vec<u64> {
	let mut rem: Vec<u64> = Vec::new();
	for i in 0u64..l {
		rem.push(i)
	}
	
	let mut p: Vec<u64> = Vec::new();
	// Loop over indices
	for i in 0u64..l {
		// Choose jth item of remaining to push
		let j = ((n-1)/factorial(l-i-1)) % (l-i);
		// Remove item from remaining and insert it into permutation
		p.push(rem.remove(j as usize));
	}
	p
}

fn factorial(n: u64) -> u64 {
	match n {
		0 => 1,
		1 => 1,
		2 => 2,
		3 => 6,
		4 => 24,
		5 => 120,
		6 => 720,
		7 => 5040,
		8 => 40320,
		9 => 362880,
		10 => 3628800,
		_ => n * factorial(n-1)
	}
}
