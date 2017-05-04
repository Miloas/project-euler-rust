// solve(600851475143)

extern crate lib;

pub fn solve(x: i64) -> i64 {
  lib::find_factor(x).pop().unwrap()
}