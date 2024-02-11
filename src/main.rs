mod io;

use num_traits::Zero;
use std::f64::consts::PI;

use crate::io::output_sequences_with_x;

const NUM_SAMPLES: usize = 8192 * 4;
const SAMPLE_RATE: f64 = 44100.0;

fn main() {
    use rustfft::{num_complex::Complex, FftPlanner};

    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(NUM_SAMPLES);

    let mut buffer = vec![Complex::zero(); NUM_SAMPLES];

    // サンプル数を44100にスケールする
    let hz = 400.0;
    // サンプリングされた時間の列を作成
    let time_seq = (0..NUM_SAMPLES)
        .map(|step| step as f64 / SAMPLE_RATE)
        .collect::<Vec<f64>>();
    // 時間列をもとにテスト波形を作成
    for (step, target) in buffer.iter_mut().enumerate() {
        let re = (2.0 * PI * hz * time_seq[step]).sin();
        *target = Complex::new(re, 0.0);
    }

    output_sequences_with_x(
        "time.dat",
        &time_seq,
        &buffer.iter().map(|v| v.re).collect::<Vec<f64>>(),
    )
    .unwrap();
    //  FFT実行（手動で正規化が必要）
    fft.process(&mut buffer);
    let buf_len = buffer.len() as f64;
    for v in buffer.iter_mut() {
        *v /= buf_len;
    }
    // 変換後の周波数系列を作成
    let freq_seq = (0..NUM_SAMPLES)
        .map(|i| i as f64 * SAMPLE_RATE / NUM_SAMPLES as f64)
        .collect::<Vec<f64>>();
    output_sequences_with_x(
        "fft.dat",
        &freq_seq,
        &buffer.iter().map(|c| c.norm()).collect::<Vec<f64>>(),
    )
    .unwrap();
}
