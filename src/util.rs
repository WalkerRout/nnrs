
use crate::matrix::Matrix;

pub fn activation(x: &Matrix) -> Matrix {
  let __activation = |x| if x > 0. {x} else {0.};
  let mut res = x.clone();

  for x in &mut res.data {
    *x = __activation(*x);
  }

  res
}

pub fn activation_prime(x: &Matrix) -> Matrix {
  let __activation_prime = |x| if x > 0. {1.} else {0.};
  let mut res = x.clone();

  for x in &mut res.data {
    *x = __activation_prime(*x);
  }

  res
}

pub fn activation_last(x: &Matrix) -> Matrix {
  let mut res = x.clone();
  let total: f64 = x.data
    .iter()
    .cloned()
    .map(f64::exp)
    .sum();

  for x in &mut res.data {
    *x = (*x).exp() / total;
  }

  res
}