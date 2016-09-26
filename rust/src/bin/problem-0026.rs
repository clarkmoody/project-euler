/// Problem 26
/// A unit fraction contains 1 in the numerator. The decimal representation of 
/// the unit fractions with denominators 2 to 10 are given:
/// 
/// 		1/2	= 	0.5
/// 		1/3	= 	0.(3)
/// 		1/4	= 	0.25
/// 		1/5	= 	0.2
/// 		1/6	= 	0.1(6)
/// 		1/7	= 	0.(142857)
/// 		1/8	= 	0.125
/// 		1/9	= 	0.(1)
/// 		1/10	= 	0.1
/// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can 
/// be seen that 1/7 has a 6-digit recurring cycle.
/// 
/// Find the value of d < 1000 for which 1/d contains the longest recurring 
/// cycle in its decimal fraction part.
fn main() {
	let mut longest = 1;
	for d in 2u16..1000u16 {
		let mut top: u16 = 10;
		let mut cycle: Vec<u16> = Vec::new();
		let bot: u16 = d;
		// let mut div = 0;
		let mut rem = 1;
		while rem > 0 {
			// div = top/bot;
			rem = top%bot;
			match cycle.iter().position(|&x| x == top) {
				Some(_) => break,
				None => cycle.push(top)
			};
			top = rem*10;
		}
		if cycle.len() > longest {
			longest = cycle.len();
			println!("1/{} cycle length = {}", d, cycle.len());
		}
	}
	println!("Answer: {}", longest);
}
