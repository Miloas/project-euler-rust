// solve(1000)
// O(1)
pub fn solve(x: u32) -> u32 {
  let (mut tmp, mut ret) = (x - 1, 0);
  while tmp % 3 != 0 { 
    tmp -= 1; 
  }
  ret += (3 + tmp) * (x - 1) / 6;
  tmp = x - 1;
  while tmp % 5 != 0 {
    tmp -= 1;
  }
  ret += (5 + tmp) * (x - 1) / 10;
  tmp = x - 1;
  while tmp % 15 != 0 {
    tmp -= 1;
  }
  ret -= (15 + tmp) * (x - 1) / 30;

  ret
}