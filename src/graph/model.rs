use nannou::{
    color::{Hue, IntoLinSrgba},
    draw::properties::ColorScalar,
    prelude::*,
};

use crate::fft::{create_shape, fft_points};

pub struct Model {
    _window: window::Id,
    fg_color: Hsl,
    shape_points: Vec<Point2>,
    fft_result: Vec<Point2>,
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let shape_points = create_shape();
    let fft_result = fft_points(&shape_points);
    let shape_points_vec2 = shape_points
        .iter()
        .map(|c| pt2(c.re as f32, c.im as f32))
        .collect();
    let fft_result_vec2 = fft_result
        .iter()
        .map(|c| pt2(c.re as f32, c.im as f32))
        .collect::<Vec<Vec2>>();
    Model {
        _window,
        fg_color: Hsl::new(0.0, 1.0, 0.3),
        shape_points: shape_points_vec2,
        fft_result: fft_result_vec2,
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    // model.start = model.end;
    // let angle = random_range(0.0, TAU);
    // let vec = vec2(angle.cos(), angle.sin()) * model.step_len;
    // model.end = model.start + vec;
    model.fg_color = model.fg_color.shift_hue(0.5);
}

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
    draw.ellipse().color(CIRCLE_COLOR).xy(center).radius(5.0);
    draw.ellipse().color(CIRCLE_COLOR).xy(end_point).radius(5.0);
    draw.line().color(CIRCLE_COLOR).start(center).end(end_point);
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // if app.elapsed_frames() == 0 {
    //     draw.background().color(model.bg_color);
    // }
    draw.background().color(WHITE);
    draw.polyline()
        .weight(2.0)
        .color(STEELBLUE)
        .points(model.shape_points.clone());
    for c in model.fft_result.iter() {
        draw_circle(&draw, *c, 2.0, true, GRAY);
    }
    draw_phase_circle(
        &draw,
        Vec2::ZERO,
        200.0,
        app.duration.since_start.as_secs_f32(),
    );
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
