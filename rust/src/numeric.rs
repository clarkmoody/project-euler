/// Solution of function by Newton's Method
pub fn fsolve<F>(f: &F, guess: f64) -> f64 
    where F : Fn(f64) -> f64 {

    let max_iter: u8 = 20;
    let tol: f64 = 1e-8;
    let mut iter: u8 = 0;
    let mut done: bool = false;

    let mut x0: f64 = guess;
    let mut x1: f64 = x0;

    while !done {
        let y: f64 = f(x0);
        let dy: f64 = nderiv(f,x0);
        // TODO: Check derivative is larger than epslion

        x1 = x0 - y/dy;

        if (x1-x0).abs() <= tol * x1.abs() {
            // Success here
            break
        }

        x0 = x1;

        iter += 1;
        if iter >= max_iter {
            done = true;
        }
    }

    x1
}

/// Numerical Derivative
pub fn nderiv<F>(f: &F, x: f64) -> f64
    where F : Fn(f64) -> f64 {

    let delta: f64 = 1e-10;
    
    (f(x+delta)-f(x-delta))/(2f64*delta)
}
