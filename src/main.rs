use num_complex::Complex;
use std::f64::consts::PI;

fn compute_j_invariant(tau: Complex<f64>) -> f64 {
    let two_pi_i = Complex::new(0.0, 2.0 * PI);
    let q = (two_pi_i * tau).exp();

    // Compute j(τ) using the truncated q-expansion
    let j_approx = 1.0 / q.re
        + 744.0
        + 196884.0 * q.re
        + 21493760.0 * q.re.powi(2)
        + 864299970.0 * q.re.powi(3);

    j_approx
}

fn main() {
    let tau1 = Complex::new(0.0, 2.0); // i√16 / 2
    let tau2 = Complex::new(0.0, 1.0); // i√16 / 4

    let j1 = compute_j_invariant(tau1);
    let j2 = compute_j_invariant(tau2);

    println!("Computed j-invariant j1: {}", j1.round());
    println!("Computed j-invariant j2: {}", j2.round());
    
}
