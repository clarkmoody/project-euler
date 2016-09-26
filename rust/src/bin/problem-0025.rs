/// Problem 25
/// The Fibonacci sequence is defined by the recurrence relation:
/// 
/// Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
/// Hence the first 12 terms will be:
/// 
/// 		F1 = 1
/// 		F2 = 1
/// 		F3 = 2
/// 		F4 = 3
/// 		F5 = 5
/// 		F6 = 8
/// 		F7 = 13
/// 		F8 = 21
/// 		F9 = 34
/// 		F10 = 55
/// 		F11 = 89
/// 		F12 = 144
///
/// The 12th term, F12, is the first term to contain three digits.
/// 
/// What is the index of the first term in the Fibonacci sequence to contain 
/// 1000 digits?
fn main() {
	let mut a: Vec<u8> = vec![1];
	let mut b: Vec<u8> = vec![1];
	let mut c: Vec<u8> = vec![2];
	let mut i: u64 = 3;
	loop {
		// Vector digit addition
		let mut rem: u8 = 0;
		c.clear();
		let mut j = 0;
		loop {
			let mut s = rem;
			if j < a.len() {
				s += a[j];
			}
			if j < b.len() {
				s += b[j];
			}
			c.push(s%10);
			rem = s / 10;
			j += 1;
			if j >= a.len() && j >= b.len() && rem == 0 {
				break
			}
		}
		if c.len() >= 1000 {
			println!("Answer: {}", i);
			// println!("{:?}", c);
			break
		}
		a = b.to_vec();
		b = c.to_vec();
		i += 1;
	}
}
