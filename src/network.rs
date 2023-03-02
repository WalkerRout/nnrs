
use crate::matrix::Matrix;
use crate::util::*;

struct Layer {
  W:  Matrix,
  b:  Matrix,
  z:  Matrix,
  a:  Matrix,
  dW: Matrix,
  db: Matrix,
  dz: Matrix,
}

pub struct Network {
  L: usize, // n.len() - 1
  error: f64,
  n: Vec<usize>,
  layers: Vec<Layer>,
}

impl Network {
  pub fn new(architecture: &[usize]) -> Self {
    let L = architecture.len() - 1;
    let error = 1.0;
    let n = architecture.to_vec();
    let mut layers = Vec::new();

    let inp = Layer {
      W:  Matrix::empty(),
      b:  Matrix::empty(),
      z:  Matrix::empty(),
      a:  Matrix::ones(architecture[0], 1),
      dW: Matrix::empty(),
      db: Matrix::empty(),
      dz: Matrix::empty(),
    };
    layers.push(inp);

    for i in 1..=L {
      let c = Layer {
        W:  Matrix::random(n[i], n[i - 1]),
        b:  Matrix::random(n[i], 1),
        z:  Matrix::ones(n[i], 1),
        a:  Matrix::ones(n[i], 1),
        dW: Matrix::ones(n[i], 1),
        db: Matrix::ones(n[i], 1),
        dz: Matrix::ones(n[i], 1),
      };
      layers.push(c);
    }

    Network { L, error, n, layers }
  }

  fn forward(&mut self, X: Matrix) {
    self.layers[0].a = X;

    for i in 1..self.L {
      self.layers[i].z = (&self.layers[i].W ^ &self.layers[i - 1].a) + self.layers[i].b.clone();
      self.layers[i].a = activation(&self.layers[i].z);
    }

    self.layers[self.L].z = (&self.layers[self.L].W ^ &self.layers[self.L - 1].a) + self.layers[self.L].b.clone();
    self.layers[self.L].a = activation_last(&self.layers[self.L].z);
  }
  
  fn error(&mut self, y: Matrix) {
    if let Some(index) = y.data.iter().position(|x| *x as i64 == 1) {
      self.error = self.layers[self.L].a.data[index].ln() * -1.0;
    } else {
      panic!("Unable to find the class!");
    }
  }

  fn delta(&mut self, y: Matrix) {
    self.layers[self.L].dz = &self.layers[self.L].a - &y;
    self.layers[self.L].dW = &self.layers[self.L].dz ^ &self.layers[self.L - 1].a.t();
    self.layers[self.L].db = self.layers[self.L].dz.clone();

    for i in (1..=self.L - 1).rev() {
      self.layers[i].dz = (&self.layers[i + 1].W.t() ^ &self.layers[i + 1].dz) * activation_prime(&self.layers[i].dz);
      self.layers[i].dW = &self.layers[i].dz ^ &self.layers[i - 1].a.t();
      self.layers[i].db = self.layers[i].dz.clone();
    }
  }

  fn gradient_descent(&mut self, alpha: f64) {
    for i in 1..=self.L {
      self.layers[i].W = &self.layers[i].W - &(&self.layers[i].dW * alpha);
      self.layers[i].b = &self.layers[i].b - &(&self.layers[i].db * alpha);
    }
  }

  pub fn predict(&mut self, X: Matrix) -> Matrix {
    self.forward(X);
    self.layers[self.L].a.clone()
  }

  pub fn fit(&mut self, mut Xs: Matrix, mut Ys: Matrix, epochs: usize, alpha: f64) {
    for _epoch in 0..epochs {
      for i in 0..Xs.rows {
        let x = Xs.row(i).t();
        let y = Ys.row(i).t();

        self.forward(x);
        self.error(y.clone());
        self.delta(y);
        self.gradient_descent(alpha);
      }
    }
  }
  
}

pub struct NetworkBuilder {
  nn_L: usize,
  nn_error: f64,
  nn_n: Vec<usize>,
  nn_layers: Vec<Layer>,
  architecture_flag: bool,
  layer_flag: bool
}

impl std::default::Default for NetworkBuilder {
  fn default() -> Self {
    NetworkBuilder { 
      nn_L: 0, 
      nn_error: 1., 
      nn_n: vec![], 
      nn_layers: vec![], 
      architecture_flag: false, 
      layer_flag: false 
    }
  }
}

impl NetworkBuilder {
  pub fn add_layer(&mut self, num_neurons: usize) -> &mut Self {
    if self.architecture_flag { return self; }
    self.nn_n.push(num_neurons);
    self.layer_flag = true;

    let layer = if self.nn_L == 0 {
      // input layer
      Layer {
        W:  Matrix::empty(),
        b:  Matrix::empty(),
        z:  Matrix::empty(),
        a:  Matrix::ones(num_neurons, 1),
        dW: Matrix::empty(),
        db: Matrix::empty(),
        dz: Matrix::empty(),
      }
    } else {
      let n = &self.nn_n;
      let i = self.nn_L;
      Layer {
        W:  Matrix::random(n[i], n[i - 1]),
        b:  Matrix::random(n[i], 1),
        z:  Matrix::ones(n[i], 1),
        a:  Matrix::ones(n[i], 1),
        dW: Matrix::ones(n[i], 1),
        db: Matrix::ones(n[i], 1),
        dz: Matrix::ones(n[i], 1),
      }
    };

    self.nn_L += 1;
    self.nn_layers.push(layer);
    self
  }

  pub fn architecture(&mut self, architecture: &[usize]) -> &mut Self {
    if self.layer_flag { return self; }
    self.architecture_flag = true;
    self.nn_L = architecture.len() - 1;
    self.nn_n = architecture.to_vec();

    let inp = Layer {
      W:  Matrix::empty(),
      b:  Matrix::empty(),
      z:  Matrix::empty(),
      a:  Matrix::ones(architecture[0], 1),
      dW: Matrix::empty(),
      db: Matrix::empty(),
      dz: Matrix::empty(),
    };
    self.nn_layers.push(inp);

    let n = &self.nn_n;
    for i in 1..=self.nn_L {
      let c = Layer {
        W:  Matrix::random(n[i], n[i - 1]),
        b:  Matrix::random(n[i], 1),
        z:  Matrix::ones(n[i], 1),
        a:  Matrix::ones(n[i], 1),
        dW: Matrix::ones(n[i], 1),
        db: Matrix::ones(n[i], 1),
        dz: Matrix::ones(n[i], 1),
      };
      self.nn_layers.push(c);
    }
    self
  }

  pub fn build(&mut self) -> Network {
    let L = if self.architecture_flag { self.nn_L } else { self.nn_L - 1 };
    assert_eq!(L, self.nn_n.len() - 1);
    assert_eq!(L, self.nn_layers.len() - 1);

    Network {
      L: L,
      error: self.nn_error,
      n: std::mem::take(&mut self.nn_n), 
      layers: std::mem::take(&mut self.nn_layers)
    }
  }
}