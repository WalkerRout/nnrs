
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
  pub rows: usize,
  pub cols: usize,
  pub data: Vec<f64>,
}

impl Matrix {
  pub fn new(rows: usize, cols: usize) -> Self {
    Matrix { rows, cols, data: vec![0.0; rows * cols] }
  }

  pub fn ones(rows: usize, cols: usize) -> Self {
    Matrix { rows, cols, data: vec![1.0; rows * cols] }
  }

  pub fn empty() -> Self {
    Matrix { rows: 0, cols: 0, data: Vec::new() }
  }

  pub fn random(rows: usize, cols: usize) -> Self {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();

    for _ in 0..rows * cols {
      data.push(rng.gen_range(-1.0..=1.0));
    }
    
    Matrix { rows, cols, data }
  }
  
  pub fn print(&self) {
    for i in 0..self.rows {
      for j in 0..self.cols {
        if let Some(value) = self.data.get(i * self.cols + j) {
          print!("{} ", value)
        }
      }
      println!();
    }
  }

  pub fn get(&self, row: usize, col: usize) -> f64 {
    if let Some(value) = self.data.get(row * self.cols + col) {
      *value
    } else {
      panic!("Unable to retrieve element! Check row and column!");
    }
  }

  pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
    if let Some(value) = self.data.get_mut(row * self.cols + col) {
      value
    } else {
      panic!("Unable to retrieve element! Check row and column!");
    }
  }

  pub fn row(&mut self, row: usize) -> Matrix {
    assert!(row < self.rows);
    
    let mut res = Vec::new();
    
    for i in 0..self.cols {
      res.push(self.get(row, i));
    }

    Matrix { rows: 1, cols: res.len(), data: res }
  }

  pub fn col(&mut self, col: usize) -> Matrix {
    assert!(col < self.cols);
    
    let mut res = Vec::new();
    
    for i in 0..self.rows {
      res.push(self.get(i, col));
    }

    Matrix { rows: res.len(), cols: 1, data: res }
  }

  pub fn sum(&self) -> f64 {
    self.data.iter().sum()
  }

  pub fn t(&self) -> Matrix {
    let mut res = Matrix::new(self.cols, self.rows);

    for i in 0..self.rows {
      for j in 0..self.cols {
        *res.get_mut(j, i) = self.get(i, j);
      }
    }

    res
  }
  
}

impl std::ops::Add<f64> for Matrix {
  type Output = Matrix;

  fn add(self, rhs: f64) -> Self::Output {
    let mut res = self;

    for x in res.data.iter_mut() {
      *x += rhs;
    }

    res
  }
}

impl std::ops::Add<Matrix> for Matrix {
  type Output = Matrix;

  fn add(self, rhs: Matrix) -> Self::Output {
    assert_eq!(self.rows, rhs.rows);
    assert_eq!(self.cols, rhs.cols);
    
    let mut res = self;

    for (i, value) in rhs.data.iter().enumerate() {
      if let Some(r) = res.data.get_mut(i) {
        *r += value;
      }
    }

    res
  }
}

impl<'a> std::ops::Add<f64> for &'a Matrix {
  type Output = Matrix;

  fn add(self, rhs: f64) -> Self::Output {
    let mut res = self.clone();

    for x in res.data.iter_mut() {
      *x += rhs;
    }

    res
  }
}

impl<'a> std::ops::Add<&'a Matrix> for &'a Matrix {
  type Output = Matrix;

  fn add(self, rhs: &'a Matrix) -> Self::Output {
    assert_eq!(self.rows, rhs.rows);
    assert_eq!(self.cols, rhs.cols);
    
    let mut res = self.clone();

    for (i, value) in rhs.data.iter().enumerate() {
      if let Some(r) = res.data.get_mut(i) {
        *r += value;
      }
    }

    res
  }
}

impl std::ops::Sub<f64> for Matrix {
  type Output = Matrix;

  fn sub(self, rhs: f64) -> Self::Output {
    let mut res = self;

    for x in res.data.iter_mut() {
      *x -= rhs;
    }

    res
  }
}

impl std::ops::Sub<Matrix> for Matrix {
  type Output = Matrix;

  fn sub(self, rhs: Matrix) -> Self::Output {
    assert_eq!(self.rows, rhs.rows);
    assert_eq!(self.cols, rhs.cols);
    
    let mut res = self;

    for (i, value) in rhs.data.iter().enumerate() {
      if let Some(r) = res.data.get_mut(i) {
        *r -= value;
      }
    }

    res
  }
}

impl<'a> std::ops::Sub<f64> for &'a Matrix {
  type Output = Matrix;

  fn sub(self, rhs: f64) -> Self::Output {
    let mut res = self.clone();

    for x in res.data.iter_mut() {
      *x -= rhs;
    }

    res
  }
}

impl<'a> std::ops::Sub<&'a Matrix> for &'a Matrix {
  type Output = Matrix;

  fn sub(self, rhs: &'a Matrix) -> Self::Output {
    assert_eq!(self.rows, rhs.rows);
    assert_eq!(self.cols, rhs.cols);
    
    let mut res = self.clone();

    for (i, value) in rhs.data.iter().enumerate() {
      if let Some(r) = res.data.get_mut(i) {
        *r -= value;
      }
    }

    res
  }
}

impl std::ops::Mul<f64> for Matrix {
  type Output = Matrix;

  fn mul(self, rhs: f64) -> Self::Output {
    let mut res = self;

    for x in res.data.iter_mut() {
      *x *= rhs;
    }

    res
  }
}

impl std::ops::Mul<Matrix> for Matrix {
  type Output = Matrix;

  fn mul(self, rhs: Matrix) -> Self::Output {
    assert_eq!(self.rows, rhs.rows);
    assert_eq!(self.cols, rhs.cols);
    
    let mut res = self;

    for (i, value) in rhs.data.iter().enumerate() {
      if let Some(r) = res.data.get_mut(i) {
        *r *= value;
      }
    }

    res
  }
}

impl<'a> std::ops::Mul<f64> for &'a Matrix {
  type Output = Matrix;

  fn mul(self, rhs: f64) -> Self::Output {
    let mut res = self.clone();

    for x in res.data.iter_mut() {
      *x *= rhs;
    }

    res
  }
}

impl<'a> std::ops::Mul<&'a Matrix> for &'a Matrix {
  type Output = Matrix;

  fn mul(self, rhs: &'a Matrix) -> Self::Output {
    assert_eq!(self.rows, rhs.rows);
    assert_eq!(self.cols, rhs.cols);
    
    let mut res = self.clone();

    for (i, value) in rhs.data.iter().enumerate() {
      if let Some(r) = res.data.get_mut(i) {
        *r *= value;
      }
    }

    res
  }
}

impl std::ops::Div<f64> for Matrix {
  type Output = Matrix;

  fn div(self, rhs: f64) -> Self::Output {
    let mut res = self;

    for x in res.data.iter_mut() {
      *x /= rhs;
    }

    res
  }
}

impl std::ops::Div<Matrix> for Matrix {
  type Output = Matrix;

  fn div(self, rhs: Matrix) -> Self::Output {
    assert_eq!(self.rows, rhs.rows);
    assert_eq!(self.cols, rhs.cols);
    
    let mut res = self;

    for (i, value) in rhs.data.iter().enumerate() {
      if let Some(r) = res.data.get_mut(i) {
        *r /= value;
      }
    }

    res
  }
}

impl<'a> std::ops::Div<f64> for &'a Matrix {
  type Output = Matrix;

  fn div(self, rhs: f64) -> Self::Output {
    let mut res = self.clone();

    for x in res.data.iter_mut() {
      *x /= rhs;
    }

    res
  }
}

impl<'a> std::ops::Div<&'a Matrix> for &'a Matrix {
  type Output = Matrix;

  fn div(self, rhs: &'a Matrix) -> Self::Output {
    assert_eq!(self.rows, rhs.rows);
    assert_eq!(self.cols, rhs.cols);
    
    let mut res = self.clone();

    for (i, value) in rhs.data.iter().enumerate() {
      if let Some(r) = res.data.get_mut(i) {
        *r /= value;
      }
    }

    res
  }
}

// THIS ACTS AS THE DOT PRODUCT OPERATOR, NOT BITXOR
impl std::ops::BitXor for Matrix {
  type Output = Matrix;

  // dotproduct
  fn bitxor(self, rhs: Matrix) -> Self::Output {
    assert_eq!(self.cols, rhs.rows);
    
    let mut res = Matrix::new(self.rows, rhs.cols);

    for i in 0..self.rows {
      for j in 0..rhs.cols {
        for k in 0..rhs.rows {
          if let Some(r) = res.data.get_mut(i * self.cols + j) {
            *r += self.get(i, k) * rhs.get(k, j);
          }
        }
      }
    }

    res
  }
}

impl<'a> std::ops::BitXor<&'a Matrix> for &'a Matrix {
  type Output = Matrix;

  // dotproduct
  fn bitxor(self, rhs: &'a Matrix) -> Self::Output {
    assert_eq!(self.cols, rhs.rows);
    
    let mut res = Matrix::new(self.rows, rhs.cols);

    for i in 0..self.rows {
      for j in 0..rhs.cols {
        for k in 0..rhs.rows {
          if let Some(r) = res.data.get_mut(i * self.cols + j) {
            *r += self.get(i, k) * rhs.get(k, j);
          }
        }
      }
    }

    res
  }
}

impl std::fmt::Display for Matrix {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut res = String::new();
    
    for i in 0..self.rows {
      for j in 0..self.cols {
        if let Some(value) = self.data.get(i * self.cols + j) {
          res.push_str(&format!("{} ", value));
        }
      }
      res.push('\n');
    }
    
    write!(f, "{}", res)
  }
}
