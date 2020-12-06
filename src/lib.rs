
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

  pub fn how_many(self: Self) -> u64 {
    return (self.n as u64).pow(self.m as u32);
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

  #[test]
  fn how_many() {
    let distr = Distribution::new(1, 1);
    assert_eq!(distr.how_many(), 1);

    let distr = Distribution::new(3, 3);
    assert_eq!(distr.how_many(), 27);

    let distr = Distribution::new(4, 3);
    assert_eq!(distr.how_many(), 64);

    let distr = Distribution::new(10, 10);
    assert_eq!(distr.how_many(), 10_000_000_000);
  }
}

