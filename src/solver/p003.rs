// solve(600851475143)

extern crate lib;

pub fn solve(x: i64) -> i64 {
  match lib::find_factor(x).pop() {
    Some(ret) => ret,
    None => -1
  }
}