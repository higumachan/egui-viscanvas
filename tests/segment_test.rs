use egui::{Color32, Pos2};
use egui_viscanvas::{Segment, SegmentAccent};

#[path = "./common.rs"]
mod common;

#[test]
fn test_basic_segment() {
    let contents = vec![
        Segment::new(Pos2::new(100.0, 100.0), Pos2::new(300.0, 300.0))
            .with_stroke_color(Color32::RED)
            .with_stroke_thickness(2.0)
            .into(),
    ];

    common::render_and_snapshot("basic_segment", &contents);
}

#[test]
fn test_segment_with_arrow() {
    let contents = vec![
        Segment::new(Pos2::new(100.0, 100.0), Pos2::new(300.0, 300.0))
            .with_stroke_color(Color32::RED)
            .with_stroke_thickness(2.0)
            .with_end_accent(SegmentAccent::Arrow)
            .into(),
    ];

    common::render_and_snapshot("segment_with_arrow", &contents);
}

#[test]
fn test_segment_with_both_arrows() {
    let contents = vec![
        Segment::new(Pos2::new(100.0, 100.0), Pos2::new(300.0, 300.0))
            .with_stroke_color(Color32::RED)
            .with_stroke_thickness(2.0)
            .with_start_accent(SegmentAccent::Arrow)
            .with_end_accent(SegmentAccent::Arrow)
            .into(),
    ];

    common::render_and_snapshot("segment_with_both_arrows", &contents);
}

#[test]
fn test_multiple_segments() {
    let contents = vec![
        Segment::new(Pos2::new(50.0, 50.0), Pos2::new(250.0, 50.0))
            .with_stroke_color(Color32::RED)
            .with_stroke_thickness(2.0)
            .into(),
        Segment::new(Pos2::new(50.0, 150.0), Pos2::new(250.0, 150.0))
            .with_stroke_color(Color32::GREEN)
            .with_stroke_thickness(3.0)
            .with_end_accent(SegmentAccent::Arrow)
            .into(),
        Segment::new(Pos2::new(50.0, 250.0), Pos2::new(250.0, 250.0))
            .with_stroke_color(Color32::BLUE)
            .with_stroke_thickness(4.0)
            .with_start_accent(SegmentAccent::Arrow)
            .with_end_accent(SegmentAccent::Arrow)
            .into(),
    ];

    common::render_and_snapshot("multiple_segments", &contents);
}
