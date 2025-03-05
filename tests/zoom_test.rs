use egui::{Color32, Pos2, Vec2};
use egui_viscanvas::{Arc, Circle, Origin, Rectangle, Segment};
use rstest::rstest;
use std::f32::consts::PI;

#[path = "common.rs"]
mod common;

#[rstest]
#[case(0.5)]
#[case(1.0)]
#[case(2.0)]
fn test_different_zoom_levels(#[case] zoom: f32) {
    // 基本的な図形を含むコンテンツを作成
    let contents = vec![
        Rectangle::new()
            .with_position(Pos2::new(100.0, 100.0))
            .with_size(Vec2::new(100.0, 100.0))
            .with_stroke_color(Color32::RED)
            .with_stroke_thickness(2.0)
            .into(),
        Circle::new()
            .with_center(Pos2::new(300.0, 150.0))
            .with_radius(50.0)
            .with_stroke_color(Color32::BLUE)
            .with_stroke_thickness(2.0)
            .into(),
        Segment::new(Pos2::new(100.0, 300.0), Pos2::new(300.0, 300.0))
            .with_stroke_color(Color32::GREEN)
            .with_stroke_thickness(2.0)
            .into(),
        Arc::new()
            .with_center(Pos2::new(200.0, 200.0))
            .with_radius(80.0)
            .with_angles(0.0, PI / 2.0) // 90度
            .with_stroke_color(Color32::YELLOW)
            .with_stroke_thickness(3.0)
            .into(),
    ];

    common::render_with_zoom_and_snapshot(&contents, zoom);
}

#[rstest]
#[case(Origin::TopLeft)]
#[case(Origin::BottomLeft)]
fn test_origin_settings(#[case] origin: Origin) {
    // 基本的な図形を含むコンテンツを作成
    let contents = vec![
        Rectangle::new()
            .with_position(Pos2::new(0.0, 0.0))
            .with_size(Vec2::new(100.0, 100.0))
            .with_stroke_color(Color32::RED)
            .with_stroke_thickness(2.0)
            .into(),
        Circle::new()
            .with_center(Pos2::new(200.0, 50.0))
            .with_radius(50.0)
            .with_stroke_color(Color32::BLUE)
            .with_stroke_thickness(2.0)
            .into(),
        Segment::new(Pos2::new(0.0, 200.0), Pos2::new(200.0, 200.0))
            .with_stroke_color(Color32::GREEN)
            .with_stroke_thickness(2.0)
            .into(),
        Arc::new()
            .with_center(Pos2::new(150.0, 150.0))
            .with_radius(70.0)
            .with_angles(PI / 4.0, PI * 3.0 / 4.0) // 45度から135度
            .with_stroke_color(Color32::YELLOW)
            .with_stroke_thickness(3.0)
            .with_label("原点テスト")
            .into(),
    ];

    // 異なる原点設定でテスト
    common::render_with_origin_and_snapshot(&contents, origin);
}
