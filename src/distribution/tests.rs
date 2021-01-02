#[cfg(test)]
mod tests {
  use crate::distribution::*;

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

