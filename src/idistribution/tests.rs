#[cfg(test)]
mod tests {
  use crate::idistribution::*;

  #[test]
  fn new() {
    let distr = IDistribution::new(1, 2);
    assert_eq!(distr.n, 1);
    assert_eq!(distr.m, 2);
  }

  #[test]
  fn how_many() {
    let distr = IDistribution::new(1, 1);
    assert_eq!(distr.how_many(), 1);

    let distr = IDistribution::new(2, 3);
    assert_eq!(distr.how_many(), 6);

    let distr = IDistribution::new(3, 2);
    assert_eq!(distr.how_many(), 4);

    let distr = IDistribution::new(3, 3);
    assert_eq!(distr.how_many(), 10);

    let distr = IDistribution::new(4, 3);
    assert_eq!(distr.how_many(), 15);
  }

  #[test]
  fn iter() {
    let distr = IDistribution::new(3, 3);
    println!("{:?}", distr);

    let mut x = 0;
    for dist_vec in &distr {
      println!("{:?}", dist_vec);
      x += 1;
    }
    assert_eq!(x, 10);

    let distr = IDistribution::new(2, 3);
    println!("{:?}", distr);

    let mut x = 0;
    for dist_vec in &distr {
      println!("{:?}", dist_vec);
      x += 1;
    }
    assert_eq!(x, 6);

    let distr = IDistribution::new(3, 2);
    println!("{:?}", distr);

    let mut x = 0;
    for dist_vec in &distr {
      println!("{:?}", dist_vec);
      x += 1;
    }
    assert_eq!(x, 4);
  }
}

