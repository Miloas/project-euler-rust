// solve(1000)
pub fn solve(n: usize) -> usize {
  for i in 1..n {
    if i * i / (n - i) * (n - i) == i * i {
      for j in 1..n {
        if j * j / (n - j) * (n - j) == j * j {
          let k = n - i - j;
          if i * i + j * j == k * k {
            return i * j * k;
          }
        }
      }
    }
  }
  0
}