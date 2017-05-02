pub fn solve() -> i32 {
  for i in 0..10 {
    for j in 0..10 {
      let a = 900009 + i * 10010 + j * 1100;
      for k in vec![1, 3, 5, 7] {
        for u in 0..10 {
            let b = 900 + u * 10 + k;
            if a == a / b * b {
              return a;
            }
        }
      }
    }
  }
  -1
}