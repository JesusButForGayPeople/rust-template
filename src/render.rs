use crate::State;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Rect, Transform};

pub fn draw_dot(pixmap: &mut Pixmap, state: &State) {
    let (x, y) = state.get_dot_position();
    let radius = 10.0;

    let mut pb = PathBuilder::new();
    pb.push_circle(x, y, radius);

    if let Some(path) = pb.finish() {
        let mut paint = Paint::default();
        paint.set_color(tiny_skia::Color::from_rgba8(242, 10, 83, 255));

        pixmap.fill_path(
            &path,
            &paint,
            tiny_skia::FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
}

pub fn draw_background(pixmap: &mut Pixmap, _state: &State) {
    let width = pixmap.width();
    let height = pixmap.height();

    let tile_size: u32 = 30; // Size of each tile in the checkered pattern

    for y in (0..height).step_by(tile_size as usize) {
        for x in (0..width).step_by(tile_size as usize) {
            let color = if (x / tile_size + y / tile_size) % 2 == 0 {
                Color::from_rgba8(89, 124, 74, 255) // Light color
            } else {
                Color::from_rgba8(119, 149, 91, 255) // Dark color
            };

            let mut paint = Paint::default();
            paint.set_color(color);

            let rect =
                Rect::from_xywh(x as f32, y as f32, tile_size as f32, tile_size as f32).unwrap();
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
        }
    }
}
