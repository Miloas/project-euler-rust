extern crate lib;

// solve(10000)
pub fn solve(n: i32) -> i64 {
  let xs = lib::primes(n);
  xs[n as usize]
}