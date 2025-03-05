use egui::{Color32, Pos2};
use egui_viscanvas::Arc;
use std::f32::consts::PI;

#[path = "./common.rs"]
mod common;

#[test]
fn test_basic_arc() {
    let contents = vec![Arc::new()
        .with_center(Pos2::new(200.0, 200.0))
        .with_radius(100.0)
        .with_angles(0.0, PI) // 半円
        .with_stroke_color(Color32::RED)
        .with_stroke_thickness(2.0)
        .into()];

    common::render_and_snapshot("basic_arc", &contents);
}

#[test]
fn test_full_circle_arc() {
    let contents = vec![Arc::new()
        .with_center(Pos2::new(200.0, 200.0))
        .with_radius(100.0)
        .with_angles(0.0, 2.0 * PI) // 完全な円
        .with_stroke_color(Color32::BLUE)
        .with_stroke_thickness(2.0)
        .into()];

    common::render_and_snapshot("full_circle_arc", &contents);
}

#[test]
fn test_labeled_arc() {
    let contents = vec![Arc::new()
        .with_center(Pos2::new(200.0, 200.0))
        .with_radius(100.0)
        .with_angles(0.0, PI / 2.0) // 90度
        .with_stroke_color(Color32::GREEN)
        .with_stroke_thickness(2.0)
        .with_label("90度")
        .into()];

    common::render_and_snapshot("labeled_arc", &contents);
}

#[test]
fn test_multiple_arcs() {
    let contents = vec![
        Arc::new()
            .with_center(Pos2::new(150.0, 150.0))
            .with_radius(50.0)
            .with_angles(0.0, PI / 2.0) // 90度
            .with_stroke_color(Color32::RED)
            .with_stroke_thickness(2.0)
            .into(),
        Arc::new()
            .with_center(Pos2::new(250.0, 150.0))
            .with_radius(50.0)
            .with_angles(PI / 2.0, PI) // 90度〜180度
            .with_stroke_color(Color32::GREEN)
            .with_stroke_thickness(2.0)
            .into(),
        Arc::new()
            .with_center(Pos2::new(200.0, 250.0))
            .with_radius(50.0)
            .with_angles(PI, 2.0 * PI) // 180度〜360度
            .with_stroke_color(Color32::BLUE)
            .with_stroke_thickness(2.0)
            .into(),
    ];

    common::render_and_snapshot("multiple_arcs", &contents);
}
