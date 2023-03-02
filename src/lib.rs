
pub mod util;
pub mod matrix;
pub mod network;

use matrix::Matrix;
use network::Network;
use network::NetworkBuilder;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_nn() {
    let rows = 7;
    let cols = 2;
    let data = vec![
      0., 0.,
      1., 0.,
      0., 1.,
      1., 1.,
      1., 0.,
      0., 0.,
      1., 1.,
    ];
    let mut xs = Matrix { rows, cols, data };

    let data = vec![
      0., 1.,
      1., 0.,
      1., 0.,
      0., 1.,
      1., 0.,
      0., 1.,
      0., 1.,
    ];
    let ys = Matrix { rows, cols, data };

    let mut nn = NetworkBuilder::default()
      .add_layer(2)
      .add_layer(12)
      .add_layer(2)
      .build();

    nn.fit(xs.clone(), ys, 1500, 0.08);
    nn.predict(xs.row(4).t()).print();

    /*
    // Alternatively using architecture style:

    let mut nn = NetworkBuilder::default()
      .architecture(&[2, 12, 2])
      .build();

    nn.fit(xs.clone(), ys, 1500, 0.08);
    nn.predict(xs.row(4).t()).print();
    */
  }
}

