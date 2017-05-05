extern crate lib;
// solve(2_000_000)
pub fn solve(n: i32) -> i64 {
  let vi = lib::primes(n / (n as f64).ln() as i32);
  let mut ret: i64 = 0;
  for x in vi {
    if x < n as i64 {
      ret += x;
    } else {
      break;
    }
  }
  ret
}