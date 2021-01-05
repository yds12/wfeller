#[path = "util.rs"] mod util;
use util::*;

/// A distribution of n indistinguishable elements in m possible cells
#[derive(Debug)]
pub struct IDistribution {
  pub n: u16,
  pub m: u16
}

pub struct IDistributionIter<'a> {
  distr: &'a IDistribution,
  index: u64
}

impl<'a> IntoIterator for &'a IDistribution {
  type Item = Vec<u16>;
  type IntoIter = IDistributionIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    IDistributionIter {
      distr: &self,
      index: 0
    }
  }
}

impl<'a> Iterator for IDistributionIter<'a> {
  type Item = Vec<u16>;

  fn next(self: &mut Self) -> Option<Self::Item> {
    if self.index < self.distr.how_many() {
      let mut distr = vec![0; self.distr.n as usize];

      self.index += 1;
      Some(distr)
    } else {
      None
    }
  }
}

impl IDistribution {
  pub fn new(n: u16, m: u16) -> Self {
    IDistribution {
      n,
      m
    }
  }

  pub fn how_many(self: &Self) -> u64 {
    return factorial(self.n + self.m - 1) / 
      (factorial(self.m - 1) * factorial(self.n));
  }
}

mod tests;

