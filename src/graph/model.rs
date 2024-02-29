use nannou::{
    color::{Hue, IntoLinSrgba},
    draw::properties::ColorScalar,
    prelude::*,
};

use crate::fft::{create_shape, fft_points};

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
    draw.ellipse().color(BLACK).xy(center).radius(5.0);
    draw.ellipse().color(CIRCLE_COLOR).xy(end_point).radius(5.0);
    draw.line().color(CIRCLE_COLOR).start(center).end(end_point);
}

struct FFTResult {
    abs: f32,
    arg: f32,
}

pub struct Model {
    _window: window::Id,
    fg_color: Hsl,
    // 点列の長さ。変更されない。
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
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let shape_points = create_shape();
    let fft_result = fft_points(&shape_points);
    let shape_points_vec2 = shape_points
        .iter()
        .map(|c| pt2(c.re as f32, c.im as f32))
        .collect();
    let seq_len = fft_result.len();
    let mut fft_results: Vec<FFTResult> = vec![];
    let mut circle_centers: Vec<Vec2> = vec![];
    let mut center = Vec2::ZERO;
    for &c in fft_result.iter() {
        let abs = c.norm() as f32 / seq_len as f32;
        let arg = c.arg() as f32;
        fft_results.push(FFTResult { abs, arg });
        circle_centers.push(center);
        center += pt2(c.re as f32, c.im as f32);
    }
    Model {
        _window,
        fg_color: Hsl::new(0.0, 1.0, 0.3),
        seq_len,
        shape_points: shape_points_vec2,
        fft_results,
        circle_centers,
        phase: 0.0,
        actual_orbit: vec![],
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    // model.line_start = model.line_end;
    // 系の位相を更新
    model.phase += TAU * 1.0 / model.seq_len as f32;
    let mut circle_centers: Vec<Vec2> = vec![];
    let mut center = Vec2::ZERO;
    for (
        idx,
        &FFTResult {
            abs: radius,
            arg: phase,
        },
    ) in model.fft_results.iter().enumerate()
    {
        circle_centers.push(center);
        let current_phase = idx as f32 * model.phase + phase;
        let next_center = radius * pt2(current_phase.cos(), current_phase.sin()) + center;
        center = next_center;
    }
    model.circle_centers = circle_centers;
    model.actual_orbit.push(center);
    // model.fg_color = model.fg_color.shift_hue(0.5);
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    // 形状を線で表示
    draw.polyline()
        .weight(2.0)
        .color(STEELBLUE)
        .points(model.shape_points.clone());
    for (idx, (&FFTResult { abs, arg }, &center)) in model
        .fft_results
        .iter()
        .zip(&model.circle_centers)
        .enumerate()
    {
        let current_phase = idx as f32 * model.phase + arg;
        draw_phase_circle(&draw, center, abs, current_phase);
    }
    draw.polyline()
        .color(ORANGERED)
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
