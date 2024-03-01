use rustfft::{num_complex::Complex, FftPlanner};

#[allow(unused)]
use crate::shapes::municipality_shape;
#[allow(unused)]
use crate::shapes::{flower, rectangle, simple_circle, ShapePoints};

/// 二次元図形を複素数で表現して与える。
/// 系列の長さは2の冪であるものとする。
pub fn create_shape() -> ShapePoints {
    // rectangle()
    // simple_circle()
    // flower()
    municipality_shape("京都府与謝郡伊根町")
}

/// 座標点列を複素関数と解釈してFFTを適用する。
pub fn fft_points(points: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let mut planner = FftPlanner::<f64>::new();
    let points_num = points.len();
    let fft = planner.plan_fft_forward(points_num);
    let mut buffer = points.to_owned();
    fft.process(&mut buffer);
    // 正規化を適用しない。する場合はこのコメントを外す
    // for v in buffer.iter_mut() {
    //     *v /= points_num as f64;
    // }
    buffer
}

/// 音っぽい周波数でFFTして周波数分布を見るテスト
#[test]
pub fn test_sound_like_freq_fft() {
    use std::f64::consts::TAU;

    use num_traits::Zero;

    use crate::io::output_sequences_with_x;

    const NUM_SAMPLES: usize = 4096;
    const SAMPLE_RATE: f64 = 44100.0;

    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(NUM_SAMPLES);

    let mut buffer = vec![Complex::zero(); NUM_SAMPLES];

    // サンプル数を44100にスケールする
    // 任意の周波数
    let hz = 10000.0;
    // サンプリングされた時間の列を作成
    let time_seq = (0..NUM_SAMPLES)
        .map(|step| step as f64 / SAMPLE_RATE)
        .collect::<Vec<f64>>();
    // 時間列をもとにテスト波形を作成
    for (step, target) in buffer.iter_mut().enumerate() {
        let re = (TAU * hz * time_seq[step]).sin();
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
