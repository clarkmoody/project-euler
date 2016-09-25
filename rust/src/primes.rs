pub fn sieve(max: u64) -> Vec<u64> {
	let mut seive: Vec<bool> = vec![true; max as usize];
	let top: u64 = (max as f64).sqrt().ceil() as u64;
	// let top: u64 = max/2;
	let mut incr: u64 = 2;
	let maxs: usize = max as usize;
	loop {
		let mut i:usize = 2*incr as usize;
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
			if ind > 0 {
				ans.push(ind as u64);
			}
		}
	}
	ans
}
