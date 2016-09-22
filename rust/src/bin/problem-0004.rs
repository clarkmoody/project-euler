/// Problem 4
///
/// A palindromic number reads the same both ways. The largest palindrome made 
/// from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
fn main() {
	println!("Problem 4");
	let mut largest: u64 = 0;
	for i in 1..999 {
		for j in 1..999 {
			let n = i*j;
			if is_palindromic(n) {
				if n > largest {
					largest = n;
				}
			}	
		}
	}
	println!("{}", largest);
}

fn is_palindromic(num: u64) -> bool {
	let s: String = format!("{}", num);
	let r: String = s.chars().rev().collect();
	s == r
}
