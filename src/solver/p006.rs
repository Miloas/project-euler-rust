pub fn solve(x: i64) -> i64 {
  let mut ret = 0;
  let x = x + 1;
  for i in 1..x {
    for j in 1..x {
      if i != j {
        ret += i * j;
      }
    }
  }
  ret
} 