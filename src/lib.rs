/// A distribution of n distinct elements in m possible cells
#[derive(Debug)]
pub struct Distribution {
  pub n: u16,
  pub m: u16
}

pub struct DistributionIter<'a> {
  distr: &'a Distribution,
  index: u64
}

impl<'a> IntoIterator for &'a Distribution {
  type Item = Vec<u16>;
  type IntoIter = DistributionIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    DistributionIter {
      distr: &self,
      index: 0
    }
  }
}

impl<'a> Iterator for DistributionIter<'a> {
  type Item = Vec<u16>;

  fn next(self: &mut Self) -> Option<Self::Item> {
    if self.index < self.distr.how_many() {
      let mut distr = vec![0; self.distr.n as usize];

      for i in 0..self.distr.n {
        distr[i as usize] = ((self.index / (self.distr.m.pow(i as u32) as u64))
          % (self.distr.m as u64)) as u16;
      }

      self.index += 1;
      Some(distr)
    } else {
      None
    }
  }
}

impl Distribution {
  pub fn new(n: u16, m: u16) -> Self {
    Distribution {
      n,
      m
    }
  }

  pub fn how_many(self: &Self) -> u64 {
    return (self.m as u64).pow(self.n as u32);
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

    let distr = Distribution::new(2, 3);
    assert_eq!(distr.how_many(), 9);

    let distr = Distribution::new(3, 2);
    assert_eq!(distr.how_many(), 8);

    let distr = Distribution::new(3, 3);
    assert_eq!(distr.how_many(), 27);

    let distr = Distribution::new(4, 3);
    assert_eq!(distr.how_many(), 81);

    let distr = Distribution::new(10, 10);
    assert_eq!(distr.how_many(), 10_000_000_000);
  }

  #[test]
  fn iter() {
    let distr = Distribution::new(3, 3);
    println!("{:?}", distr);

    let mut x = 0;
    for dist_vec in &distr {
      println!("{:?}", dist_vec);
      x += 1;
    }
    assert_eq!(x, 27);

    let distr = Distribution::new(2, 3);
    println!("{:?}", distr);

    let mut x = 0;
    for dist_vec in &distr {
      println!("{:?}", dist_vec);
      x += 1;
    }
    assert_eq!(x, 9);

    let distr = Distribution::new(3, 2);
    println!("{:?}", distr);

    let mut x = 0;
    for dist_vec in &distr {
      println!("{:?}", dist_vec);
      x += 1;
    }
    assert_eq!(x, 8);
  }
}

