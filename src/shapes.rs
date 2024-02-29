use std::f64::consts::TAU;

use num_traits::Zero;
use rustfft::num_complex::Complex;

pub type ShapePoints = Vec<Complex<f64>>;
const NUM_SAMPLES: usize = 128;

pub fn simple_circle() -> ShapePoints {
    let mut points = Vec::<Complex<f64>>::new();
    for idx in 0..NUM_SAMPLES {
        let phase = TAU * idx as f64 / (NUM_SAMPLES - 1) as f64;
        points.push(Complex::new(phase.cos(), phase.sin()) * 200.0);
    }
    points
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
