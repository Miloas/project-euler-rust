#![allow(dead_code)]

extern crate rand;

fn _rand() -> i32 {
  let mut r = rand::random::<i32>();
  if r < 0 {
    r = -r;
  }
  r
}

pub fn gcd(x: i32, y: i32) -> i32 {
  let (mut a, mut b) = if x < y {
    (y, x)
  } else {
    (x, y)
  };
  while b != 0 {
    let r = a % b;
    a = b; 
    b = r;    
  }
  a
}

#[test]
fn gcd_spec() {
  assert_eq!(2, gcd(4, 6));
  assert_eq!(3, gcd(3, 0));
}

// (x * y) mod z
pub fn mult_mod(x: i32, y: i32, z: i32) -> i32 {
  let (mut a, mut b, c, mut ret) = (x, y, z, 0);
  a %= c;
  b %= c;
  while b != 0 {
    if b & 1 != 0 {
      ret += a;
      ret %= c;
    }
    a <<= 1;
    if a >= c {
      a -= c;
    }
    b >>= 1;
  }
  ret
}

#[test]
fn mult_mod_spec() {
  assert_eq!(4, mult_mod(4, 6, 5));
}

// x^y mod z
pub fn pow_mod(x: i32, y: i32, z: i32) -> i32 {
  let (mut a, mut b, c, mut ret) = (x, y, z, 1);
  if b == 1 {
    return a % c
  }
  a %= c;
  while b != 0 {
    if b & 1 != 0 {
      ret = mult_mod(ret, a, c);
    }
    a = mult_mod(a, a, c);
    b >>= 1;
  }
  ret
}

#[test]
fn pow_mod_spec() {
  assert_eq!(3, pow_mod(2, 3, 5));
  assert_eq!(2, pow_mod(2, 1, 5));
}

// 素性二次探测: Miller-Rabin test 的helper函数
// http://www.matrix67.com/blog/archives/234
fn _check(a: i32, n: i32, x: i32, t: i32) -> bool {
  let (mut ret, mut i)= (pow_mod(a, x, n), 1);
  let mut last = ret;
  while i <= t {
    i += 1;
    ret = mult_mod(ret, ret, n);
    if ret == 1 && last != 1 && last != n-1 {
      return true;
    }
    last = ret;
  }
  if ret != 1 {
    true
  } else {
    false
  }
} 

pub fn miller_rabin(n: i32) -> bool {
  // 测试次数
  const TIMES: i32 = 20;
  if n < 2 {
    return false;
  }
  if n == 2 {
    return true;
  }
  if n & 1 == 0 {
    return false;
  }
  let (mut x, mut t, mut i) = (n-1, 0, 0);
  while x & 1 == 0 {
    x >>= 1;
    t += 1;
  }
  while i < TIMES  {
    i += 1;
    let a = _rand();
    if _check(a % (n - 1) + 1, n, x, t) {
      return false;
    }
  }
  true
}


#[test]
fn miller_rabin_spec() {
  assert_eq!(true, miller_rabin(19930507));
  assert_eq!(false, miller_rabin(19920809));
  assert_eq!(false, miller_rabin(99));
}

// xjb找x的因子,但是根据数学爸爸的定理,它很快
fn pollard_rho(n: i32) -> i32 {
  let (mut i, mut t, mut c) = (1, 2, 0);
  let mut a = _rand() % (n - 1) + 1;
  let mut b = a;
  while c == 0 || c == 2 {
    c = _rand() % (n - 1) + 1;
  }
  loop {
    i += 1;
    let p = gcd(n + b - a, n);
    if p > 1 && p < n {
      return p;
    }
    if i == t {
      b = a;
      t <<= 1;
    }
    a = (mult_mod(a, a, n) + n - c) % n;
    if a == b {
      break;
    }
  }
  n
}


pub fn find_factor(n: i32) -> Vec<i32> {
  let mut ret = Vec::new();
  _find_factor_helper(n, &mut ret);
  ret
}

fn _find_factor_helper(n: i32, ret: &mut Vec<i32>) {
  if n < 2 {
    return;
  }
  if miller_rabin(n) {
    ret.push(n);
    return;
  }
    // p = pollard_rho(p, _rand() % (n - 1) + 1);
  let p = pollard_rho(n);
  _find_factor_helper(p, ret);
  _find_factor_helper(n / p, ret);
}

#[test]
fn find_factor_spec() {
  assert_eq!(find_factor(4), [2, 2]);
  assert_eq!(find_factor(19920809), [47, 423847]);
}
