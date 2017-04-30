// solve(4_000_000)
// O(n)
pub fn solve(x: u32) -> u32 {
  let (mut fib, mut ret) = ((1, 2), 0); 
  while fib.1 <= x {
    ret += even_value(fib.1);
    fib = (fib.1, fib.0 + fib.1)
  }
  ret
}

fn even_value(x: u32) -> u32 {
  if x % 2 == 0 {
    x
  } else {
    0
  }
}