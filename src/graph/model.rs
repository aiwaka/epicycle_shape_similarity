use nannou::{
    color::{Hue, IntoLinSrgba},
    draw::properties::ColorScalar,
    prelude::*,
};

use crate::fft::{create_shape, fft_points};

const LOW_PASS_RATE: f32 = 0.5;

/// 円を描く
fn draw_circle<C>(draw: &Draw, center: Vec2, radius: f32, fill: bool, color: C)
where
    C: IntoLinSrgba<ColorScalar>,
{
    let skelton = draw.ellipse().radius(radius).xy(center);
    if fill {
        skelton.color(color);
    } else {
        skelton.stroke(color).stroke_weight(2.0).no_fill();
    }
}

/// 位相を示すバー付きの円
fn draw_phase_circle(draw: &Draw, center: Vec2, radius: f32, phase: f32) {
    const CIRCLE_COLOR: Srgb<u8> = GRAY;
    draw_circle(draw, center, radius, false, CIRCLE_COLOR);
    let end_point = radius * pt2(phase.cos(), phase.sin()) + center;
    draw.ellipse().color(GRAY).xy(center).radius(3.0);
    draw.ellipse().color(CIRCLE_COLOR).xy(end_point).radius(3.0);
    draw.line().color(CIRCLE_COLOR).start(center).end(end_point);
}

struct FFTResult {
    freq: u32,
    abs: f32,
    arg: f32,
}

pub struct Model {
    _window: window::Id,
    fg_color: Hsl,
    // もとの点列の長さ。変更されない。
    raw_seq_len: usize,
    // 削減後の点列の長さ。変更されない。
    #[allow(unused)]
    seq_len: usize,
    // 形状を表す点列。変更されない。
    shape_points: Vec<Point2>,
    // 複素数を大きさと偏角に変換して保持する。変更されない。
    fft_results: Vec<FFTResult>,
    // この各フレームで更新される各円の中心座標
    circle_centers: Vec<Vec2>,
    // 各フレームにおける位相
    phase: f32,
    // 実際に円により描かれる線を表す点列
    actual_orbit: Vec<Vec2>,
    // 一周したフラグ
    round_once: bool,
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let shape_points = create_shape();
    // 点の数を計算
    let raw_seq_len = shape_points.len();
    let seq_len = (LOW_PASS_RATE * raw_seq_len as f32) as usize;
    // FFTした上で大きさ降順にソートし、影響の小さい円から指定された割合だけ削る
    let mut fft_result = fft_points(&shape_points)
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();
    fft_result.sort_by(|(_, a), (_, b)| b.norm().total_cmp(&a.norm()));
    let fft_result = fft_result.into_iter().take(seq_len).collect::<Vec<_>>();
    // 複素点列で表された形状をVec2に変換しておく
    let shape_points_vec2 = shape_points
        .into_iter()
        .map(|c| pt2(c.re as f32, c.im as f32))
        .collect();

    let mut fft_results: Vec<FFTResult> = vec![];
    let mut circle_centers: Vec<Vec2> = vec![];
    let mut center = Vec2::ZERO;
    for (freq, c) in fft_result.into_iter() {
        let abs = c.norm() as f32 / raw_seq_len as f32;
        let arg = c.arg() as f32;
        fft_results.push(FFTResult {
            freq: freq as u32,
            abs,
            arg,
        });
        circle_centers.push(center);
        center += pt2(c.re as f32, c.im as f32);
    }
    Model {
        _window,
        fg_color: Hsl::new(0.0, 1.0, 0.6),
        raw_seq_len,
        seq_len,
        shape_points: shape_points_vec2,
        fft_results,
        circle_centers,
        phase: 0.0,
        actual_orbit: vec![],
        round_once: false,
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.round_once {
        let mut circle_centers: Vec<Vec2> = vec![];
        let mut center = Vec2::ZERO;
        for &FFTResult {
            freq,
            abs: radius,
            arg: phase,
        } in model.fft_results.iter()
        {
            circle_centers.push(center);
            let current_phase = freq as f32 * model.phase + phase;
            let next_center = radius * pt2(current_phase.cos(), current_phase.sin()) + center;
            center = next_center;
        }
        model.circle_centers = circle_centers;
        model.actual_orbit.push(center);
        // model.fg_color = model.fg_color.shift_hue(10.0);
    }
    if model.phase <= TAU {
        // 系の位相を更新
        model.phase += TAU / model.raw_seq_len as f32;
    } else {
        model.round_once = true;
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    // 形状を線で表示
    draw.polyline()
        .weight(4.0)
        .color(STEELBLUE)
        .points(model.shape_points.clone());
    if !model.round_once {
        for (&FFTResult { freq, abs, arg }, &center) in
            model.fft_results.iter().zip(&model.circle_centers)
        {
            let current_phase = freq as f32 * model.phase + arg;
            draw_phase_circle(&draw, center, abs, current_phase);
        }
    }
    draw.polyline()
        .color(model.fg_color)
        .weight(2.0)
        .points(model.actual_orbit.clone());
    // let radius = 150.0;
    // let sin = app.time.sin();
    // let sin2 = (app.time / 4.0).sin();
    // let window = app.window_rect();
    // let center_x = map_range(sin, -1.0, 1.0, window.left(), window.right());
    // let center_y = map_range(sin2, -1.0, 1.0, window.bottom(), window.top());
    // draw_circle(&draw, pt2(center_x, center_y), radius, false);

    // draw.line()
    //     .color(model.fg_color)
    //     .start(model.start)
    //     .end(model.end);
    draw.to_frame(app, &frame).unwrap();
}
