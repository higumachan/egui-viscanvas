use egui::{Color32, Pos2};
use egui_viscanvas::Circle;

#[path = "./common.rs"]
mod common;

#[test]
fn test_basic_circle() {
    let contents = vec![Circle::new()
        .with_center(Pos2::new(200.0, 200.0))
        .with_radius(100.0)
        .with_stroke_color(Color32::RED)
        .with_stroke_thickness(2.0)
        .into()];

    common::render_and_snapshot("basic_circle", &contents);
}

#[test]
fn test_filled_circle() {
    let contents = vec![Circle::new()
        .with_center(Pos2::new(200.0, 200.0))
        .with_radius(100.0)
        .with_fill_color(Color32::from_rgb(0, 128, 255))
        .with_stroke_color(Color32::RED)
        .with_stroke_thickness(2.0)
        .into()];

    common::render_and_snapshot("filled_circle", &contents);
}

#[test]
fn test_labeled_circle() {
    let contents = vec![Circle::new()
        .with_center(Pos2::new(200.0, 200.0))
        .with_radius(100.0)
        .with_stroke_color(Color32::RED)
        .with_stroke_thickness(2.0)
        .with_label("テスト円")
        .into()];

    common::render_and_snapshot("labeled_circle", &contents);
}

#[test]
fn test_multiple_circles() {
    let contents = vec![
        Circle::new()
            .with_center(Pos2::new(100.0, 100.0))
            .with_radius(50.0)
            .with_stroke_color(Color32::RED)
            .with_stroke_thickness(2.0)
            .into(),
        Circle::new()
            .with_center(Pos2::new(250.0, 100.0))
            .with_radius(50.0)
            .with_fill_color(Color32::GREEN)
            .into(),
        Circle::new()
            .with_center(Pos2::new(175.0, 250.0))
            .with_radius(50.0)
            .with_stroke_color(Color32::BLUE)
            .with_stroke_thickness(3.0)
            .with_label("中央")
            .into(),
    ];

    common::render_and_snapshot("multiple_circles", &contents);
}
