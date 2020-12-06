
/// A distribution of n distinct elements in m possible cells
pub struct Distribution {
  n: u16,
  m: u16
}

impl Distribution {
  pub fn new(n: u16, m: u16) -> Self {
    Distribution {
      n: n,
      m: m
    }
  }

  pub fn how_many(self: Self) -> u32 {
    return 0;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let distr = Distribution::new(1, 2);
    assert_eq!(distr.n, 1);
    assert_eq!(distr.m, 2);
  }
}

