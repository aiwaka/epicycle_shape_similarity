use nannou::{
    color::{Hue, IntoLinSrgba},
    draw::properties::ColorScalar,
    prelude::*,
};

use crate::fft::{create_shape, fft_points};

pub struct Model {
    _window: window::Id,
    bg_color: Srgb<u8>,
    fg_color: Hsl,
    step_len: f32,
    start: Point2,
    end: Point2,
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
        .map(|c| 200.0 * pt2(c.re as f32, c.im as f32))
        .collect::<Vec<Vec2>>();
    Model {
        _window,
        bg_color: WHITE,
        fg_color: Hsl::new(0.0, 1.0, 0.3),
        step_len: 10.0,
        start: Point2::ZERO,
        end: Point2::ZERO,
        shape_points: shape_points_vec2,
        fft_result: fft_result_vec2,
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.start = model.end;
    let angle = random_range(0.0, 2.0 * PI);
    let vec = vec2(angle.cos(), angle.sin()) * model.step_len;
    model.end = model.start + vec;
    model.fg_color = model.fg_color.shift_hue(0.5);
}

/// 円を描く
fn draw_circle<C>(draw: &Draw, center: Vec2, radius: f32, fill: bool, color: C)
where
    C: IntoLinSrgba<ColorScalar>,
{
    let [center_x, center_y] = center.to_array();
    let points = (0..=360).step_by(2).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * radius + center_x;
        let y = radian.cos() * radius + center_y;
        pt2(x, y)
    });
    if fill {
        draw.polygon().points(points).color(color);
    } else {
        draw.polyline().weight(2.0).points(points).color(color);
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // if app.elapsed_frames() == 0 {
    //     draw.background().color(model.bg_color);
    // }
    draw.background().color(model.bg_color);
    for c in model.shape_points.iter() {
        draw_circle(&draw, *c, 2.0, true, STEELBLUE);
    }
    for c in model.fft_result.iter() {
        draw_circle(&draw, *c, 2.0, true, GRAY);
    }
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
