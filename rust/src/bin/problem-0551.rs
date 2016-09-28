/// Problem 551
/// Let a0, a1, a2, ... be an integer sequence defined by:
/// 
///     a_0 = 1;
///     for n ≥ 1, a_n is the sum of the digits of all preceding terms.
///
/// The sequence starts with 1, 1, 2, 4, 8, 16, 23, 28, 38, 49, ...
/// You are given a_10^6 = 31054319.
/// 
///  a_10^1 = 62
///  a_10^2 = 1205
///  a_10^3 = 16577
///  a_10^4 = 213677
///  a_10^5 = 2609882
///  a_10^6 = 31054319
///  a_10^7 = 355356611
///  a_10^8 = 4047602471
///  a_10^9 = 45063267434
/// a_10^10 = 500834734271
///
/// Find a_10^15.
fn main() {
    // In essence, the next term is the current term plus the sum of the
    // current term's digits. a_7 = 28, sum of digits = 10, a_8 = 38.
    let mut i: u64 = 1;
    let mut a: u64 = 1;
    // let mut i: u64 = 1000000;
    // let mut a: u64 = 31054319;
    let mut c: u64 = 0;
    let mut pow: u64 = 10;
    loop {
        // if c > 25 {
        //     break
        // }
        // if i == 1000000 {
        // if i == 1000000000000000 {
        if i % pow == 0 {
            println!("a_{} = {}", i, a);
            pow *= 10;
            // break
        }
        // Take sum of digits of current term
        let mut b = a;
        let mut s = b%10;
        while b > 0 {
            b /= 10;
            s += b%10;
        }
        // Increment term
        a += s;
        i += 1;
        c += 1;
    }
}