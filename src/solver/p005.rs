
extern crate lib;

// solve(20)
pub fn solve(x: i32) -> i64 {
  let mut a = 1;
  for b in 2..(x + 1) {
    a = _lcm(a, b as i64);
  } 
  a
}

fn _lcm(a: i64, b: i64) -> i64 {
  a * b / lib::gcd(a, b)
}