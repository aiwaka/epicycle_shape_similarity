mod fft;
mod graph;
mod io;

use graph::model::{model, update};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

/// `create_shape`関数で作られた形にFFTを適用して結果をファイルに出力する。
/// `gnuplot`でプロットして確かめられる。
#[test]
fn test_basic_shape_fft() {
    use fft::{create_shape, fft_points};
    use io::output_2d_sequences;

    let shape_points = create_shape();
    let fft_result = fft_points(&shape_points);
    output_2d_sequences(
        "shape.dat",
        &shape_points
            .iter()
            .map(|c| [c.re, c.im])
            .collect::<Vec<[_; 2]>>(),
    )
    .unwrap();
    output_2d_sequences(
        "result_fft_2d.dat",
        &fft_result
            .iter()
            .map(|c| [c.re, c.im])
            .collect::<Vec<[f64; 2]>>(),
    )
    .unwrap();
}
