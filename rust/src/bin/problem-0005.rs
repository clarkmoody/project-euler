/// Problem 5
///
/// 2520 is the smallest number that can be divided by each of the numbers from 
/// 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible by all of the 
/// numbers from 1 to 20?
fn main() {
	println!("Problem 5");
	let num: u64 = 20;
	let mut vals: Vec<u64> = Vec::new();
	for i in 2..(num+1) {
		vals.push(i);
	}
	vals.reverse();
	let l: usize = vals.len();

	let mut ans: u64 = 1;
	for i in 0..l {
		println!("Vals: {:?}", vals);
		ans *= vals[i];
		let fac: Vec<u64> = factors(vals[i]);
		println!("Factors: {:?}", fac);
		for j in (i+1)..l {
			for f in &fac {
				if vals[j] % f == 0 {
					vals[j] = vals[j] / f;
				}
			}
		}
	}
	println!("Answer: {}", ans);
}

fn factors(num: u64) -> Vec<u64> {
	let mut ans: Vec<u64> = Vec::new();
	let half: u64 = ((num as f64)/2.0).ceil() as u64 + 1;
	for i in 1..half {
		if num % i == 0 {
			if !ans.contains(&i) {
				ans.push(i);
			}
			if !ans.contains(&(num/i)) {
				ans.push(num/i)
			}
		}
	}
	ans.sort_by(|a, b| b.cmp(a));
	ans
}
