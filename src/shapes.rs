use std::f64::consts::TAU;

use rustfft::num_complex::Complex;

use crate::{
    io::read_municipalities_boundary_data,
    municipalities::utils::{convert_to_shape, normalize_shape},
};

pub type ShapePoints = Vec<Complex<f64>>;
const NUM_SAMPLES: usize = 128;

pub fn simple_circle() -> ShapePoints {
    (0..NUM_SAMPLES)
        .map(|idx| 200.0 * Complex::cis(TAU * idx as f64 / (NUM_SAMPLES - 1) as f64))
        .collect()
}

pub fn rectangle() -> ShapePoints {
    use std::f64::consts::FRAC_PI_2;
    let mut points = Vec::<Complex<f64>>::new();
    for idx in 0..NUM_SAMPLES {
        let phase = TAU * idx as f64 / (NUM_SAMPLES - 1) as f64;
        let re = phase.cos();
        let im = phase.sin();

        let c = if re > im {
            if re > -im {
                Complex::new(1.0, phase.tan())
            } else {
                Complex::new((FRAC_PI_2 + phase).tan(), -1.0)
            }
        } else if re > -im {
            Complex::new((FRAC_PI_2 - phase).tan(), 1.0)
        } else {
            Complex::new(-1.0, -phase.tan())
        } * 100.0;
        points.push(c)
    }
    points
}

pub fn flower() -> ShapePoints {
    let mut points = Vec::<Complex<f64>>::new();
    for idx in 0..NUM_SAMPLES {
        let phase = TAU * idx as f64 / (NUM_SAMPLES - 1) as f64;
        let c = 150.0 * Complex::cis(phase) + 50.0 * Complex::cis(phase * 5.0);
        points.push(c)
    }
    points
}

pub fn municipality_shape() -> ShapePoints {
    let json_data = read_municipalities_boundary_data().unwrap();
    // 姫路市
    let shape = convert_to_shape(&json_data.features[194], 700);
    normalize_shape(shape)
}
