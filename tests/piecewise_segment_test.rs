use egui::{Color32, Pos2};
use egui_viscanvas::PiecewiseSegment;

#[path = "./common.rs"]
mod common;

#[test]
fn test_basic_piecewise_segment() {
    let points = vec![
        Pos2::new(100.0, 100.0),
        Pos2::new(200.0, 150.0),
        Pos2::new(300.0, 100.0),
        Pos2::new(400.0, 200.0),
    ];

    let contents = vec![PiecewiseSegment::new(points)
        .unwrap()
        .with_stroke_color(Color32::RED)
        .with_stroke_thickness(2.0)
        .into()];

    common::render_and_snapshot("basic_piecewise_segment", &contents);
}

#[test]
fn test_multiple_piecewise_segments() {
    let points1 = vec![
        Pos2::new(50.0, 50.0),
        Pos2::new(150.0, 100.0),
        Pos2::new(250.0, 50.0),
    ];

    let points2 = vec![
        Pos2::new(50.0, 200.0),
        Pos2::new(150.0, 250.0),
        Pos2::new(250.0, 200.0),
        Pos2::new(350.0, 300.0),
    ];

    let contents = vec![
        PiecewiseSegment::new(points1)
            .unwrap()
            .with_stroke_color(Color32::RED)
            .with_stroke_thickness(2.0)
            .into(),
        PiecewiseSegment::new(points2)
            .unwrap()
            .with_stroke_color(Color32::BLUE)
            .with_stroke_thickness(3.0)
            .into(),
    ];

    common::render_and_snapshot("multiple_piecewise_segments", &contents);
}

#[test]
fn test_closed_piecewise_segment() {
    let mut points = vec![
        Pos2::new(200.0, 100.0),
        Pos2::new(300.0, 100.0),
        Pos2::new(300.0, 200.0),
        Pos2::new(200.0, 200.0),
    ];
    // 閉じた図形にするために最初の点を最後にも追加
    points.push(points[0]);

    let contents = vec![PiecewiseSegment::new(points)
        .unwrap()
        .with_stroke_color(Color32::GREEN)
        .with_stroke_thickness(2.0)
        .into()];

    common::render_and_snapshot("closed_piecewise_segment", &contents);
}
