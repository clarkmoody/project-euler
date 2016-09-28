pub fn sieve(max: u64) -> Vec<u64> {
	let sb: Vec<bool> = sieve_bools(max);
	let mut sve: Vec<u64> = Vec::new();
	for (ind, val) in sb.iter().enumerate() {
		if *val { 
			if ind > 1 {
				sve.push(ind as u64);
			}
		}
	}
	sve
}

pub fn sieve_bools(max: u64) -> Vec<bool> {
	let mut sve: Vec<bool> = vec![true; max as usize];
	let top: u64 = (max as f64).sqrt().ceil() as u64;
	// let top: u64 = max/2;
	let mut incr: u64 = 2;
	let maxs: usize = max as usize;
	loop {
		let mut i:usize = 2*incr as usize;
		loop {
			sve[i] = false;
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
	sve
}
