use egui::{Color32, Pos2, Vec2};
use egui_viscanvas::Rectangle;

#[path = "./common.rs"]
mod common;

#[test]
fn test_basic_rectangle() {
    let contents = vec![Rectangle::new()
        .with_position(Pos2::new(100.0, 100.0))
        .with_size(Vec2::new(200.0, 150.0))
        .with_stroke_color(Color32::RED)
        .with_stroke_thickness(2.0)
        .into()];

    common::render_and_snapshot("basic_rectangle", &contents);
}

#[test]
fn test_filled_rectangle() {
    let contents = vec![Rectangle::new()
        .with_position(Pos2::new(100.0, 100.0))
        .with_size(Vec2::new(200.0, 150.0))
        .with_fill_color(Color32::from_rgb(0, 128, 255))
        .with_stroke_color(Color32::RED)
        .with_stroke_thickness(2.0)
        .into()];

    common::render_and_snapshot("filled_rectangle", &contents);
}

#[test]
fn test_labeled_rectangle() {
    let contents = vec![Rectangle::new()
        .with_position(Pos2::new(100.0, 100.0))
        .with_size(Vec2::new(200.0, 150.0))
        .with_stroke_color(Color32::RED)
        .with_stroke_thickness(2.0)
        .with_label("テスト矩形")
        .into()];

    common::render_and_snapshot("labeled_rectangle", &contents);
}

#[test]
fn test_multiple_rectangles() {
    let contents = vec![
        Rectangle::new()
            .with_position(Pos2::new(50.0, 50.0))
            .with_size(Vec2::new(100.0, 100.0))
            .with_stroke_color(Color32::RED)
            .with_stroke_thickness(2.0)
            .into(),
        Rectangle::new()
            .with_position(Pos2::new(200.0, 50.0))
            .with_size(Vec2::new(100.0, 100.0))
            .with_fill_color(Color32::GREEN)
            .into(),
        Rectangle::new()
            .with_position(Pos2::new(125.0, 200.0))
            .with_size(Vec2::new(100.0, 100.0))
            .with_stroke_color(Color32::BLUE)
            .with_stroke_thickness(3.0)
            .with_label("中央")
            .into(),
    ];

    common::render_and_snapshot("multiple_rectangles", &contents);
}
