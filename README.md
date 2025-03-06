# Rust Implementation of j-Invariant Computation in Finite_Fields

This Rust program computes an approximation of the modular j-invariant using a truncated q-expansion.

## Description
The j-invariant is an important function in the theory of elliptic curves and modular forms. This implementation calculates an approximation of the j-invariant given a complex number $\tau$ using the formula:

$j(\tau) \approx \frac{1}{q} + 744 + 196884q + 21493760q^2 + 864299970q^3$


where $q = e^{2\pi i \tau}$.

## Dependencies
Ensure you have Rust installed. This program requires the `num-complex` crate:

```toml
[dependencies]
num-bigint = { version = "0.4", features = ["std"] }
num-traits = "0.2"
num-integer = "0.1"
num-complex = "0.4"
```

## Usage
Clone the repository and run the program with Cargo:

```sh
git clone https://github.com/cypriansakwa/Rust_Implementation_of_j_Invariant_Computation_in_Finite_Fields.git
cd Rust_Implementation_of_j_Invariant_Computation_in_Finite_Fields
cargo run
```

## Code Explanation
- `compute_j_invariant`: Computes the j-invariant using the truncated q-expansion.
- `main`: Computes the j-invariant for two different values of $\tau$ and prints the results.

## Example Output
```sh
Computed j-invariant j1: 287496.0
Computed j-invariant j2: 1728.0
```

