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

  #[test]
  fn set_to_distr() {
    let sep = vec![0,0];
    let n = 3;
    let res = sep_to_distr(sep, n);

    assert_eq!(res, vec![0, 0, 3]);

    let sep = vec![1,2];
    let n = 3;
    let res = sep_to_distr(sep, n);

    assert_eq!(res, vec![1, 1, 1]);

    let sep = vec![0,3,9];
    let n = 10;
    let res = sep_to_distr(sep, n);

    assert_eq!(res, vec![0, 3, 6, 1]);
  }

  #[test]
  #[should_panic]
  fn set_to_distr_bad_sep() {
    let sep = vec![0,1,1,2,8,7];
    let n = 20;
    sep_to_distr(sep, n);
  }

  #[test]
  #[should_panic]
  fn set_to_distr_large_sep() {
    let sep = vec![0,1,1,2,8];
    let n = 7;
    sep_to_distr(sep, n);
  }
}

