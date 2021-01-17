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

/// We can think of these distributions of n elements as a sequence of n
/// elements with m - 1 separators inserted between them. For example, two ways
/// of distributing 3 elements in 2 boxes is:
///
/// ,*** (all elements in the second box)
/// *,** (one element in the first and 2 elements in the second box)
///
/// This function converts a vector with the index of the separators into
/// a vector of size m with the number of elements in each box. Parameter
/// `n` is the total number of elements.
fn sep_to_distr(sep: Vec<u16>, n: u16) -> Vec<u16> {
  let mut vec = vec![0; sep.len() + 1];
  let mut total = 0;

  for i in 0..sep.len() {
    if sep[i] > n {
      panic!("Bug: separator beyond n!");
    }

    if i == 0 {
      vec[i] = sep[i];
    } else {
      if sep[i] < sep[i - 1] {
        panic!("Bug: bad separator vector!");
      }

      vec[i] = sep[i] - sep[i - 1];
    }

    total += vec[i];
  }

  vec[sep.len()] = n - total;

  vec
}

fn distr_to_sep(distr: Vec<u16>) -> Vec<u16> {
  let mut sep = vec![0; distr.len() - 1];
  sep
}

mod tests;

