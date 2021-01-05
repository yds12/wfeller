pub fn factorial(n: u16) -> u64 {
  match n {
    0 => 1,
    _ => factorial(n - 1) * n as u64
  }
}
