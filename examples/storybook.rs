use eframe::NativeOptions;
use egui::ImageSource;
use egui_storybook::story_book::{Story, StoryBookBuilder};
use egui_storybook::{run_story_book, story_body};

fn main() {
    let story_book = StoryBookBuilder::new()
        .add_story(Story::new(
            "hello_canvas",
            story_body! {
                use egui::Id;
                use egui_viscanvas::*;

                let contents = vec![
                    Rectangle::new()
                        .with_position(egui::Pos2::new(0.0, 0.0))
                        .with_size(egui::Vec2::new(100.0, 100.0))
                        .with_stroke_color(egui::Color32::from_rgb(255, 0, 0))
                        .with_stroke_thickness(2.0).into(),
                ];

                egui::CentralPanel::default().show(ctx, |ui| {
                    vis_canvas(ui, Id::new("canvas"), &contents).unwrap();
                });
            },
        ))
        .add_story(Story::new(
            "labeled_rectangle_canvas",
            story_body! {
                use egui::Id;
                use egui_viscanvas::*;

                let contents = vec![
                    Rectangle::new()
                        .with_position(egui::Pos2::new(0.0, 0.0))
                        .with_size(egui::Vec2::new(100.0, 100.0))
                        .with_stroke_color(egui::Color32::from_rgb(255, 0, 0))
                        .with_label("Hello")
                        .with_stroke_thickness(2.0).into(),
                ];

                egui::CentralPanel::default().show(ctx, |ui| {
                    vis_canvas(ui, Id::new("canvas"), &contents).unwrap();
                });
            },
        ))
        .add_story(
            Story::new(
                "image_canvas",
                story_body! {
                    use egui::{Id, ImageSource};
                    use egui_viscanvas::*;

                    let contents = vec![
                        Image::new(ImageSource::from(("bytes://logo.png", include_bytes!("../assets/logo.png"))))
                            .into(),
                    ];

                    egui::CentralPanel::default().show(ctx, |ui| {
                        vis_canvas(ui, Id::new("canvas"), &contents).unwrap();
                    });
                },
            )
            .add_asset_file("./assets/logo.png".into()),
        )
        .add_story(
            Story::new(
                "composed_canvas",
                story_body! {
                    use egui::{Id, ImageSource};
                    use egui_viscanvas::*;

                    let contents = vec![
                        Image::new(ImageSource::from(("bytes://logo.png", include_bytes!("../assets/logo.png"))))
                            .into(),
                        Rectangle::new()
                            .with_position(egui::Pos2::new(100.0, 100.0))
                            .with_size(egui::Vec2::new(256.0, 256.0))
                            .with_stroke_color(egui::Color32::from_rgb(155, 155, 0))
                            .with_stroke_thickness(2.0).into(),
                    ];

                    egui::CentralPanel::default().show(ctx, |ui| {
                        vis_canvas(ui, Id::new("canvas"), &contents).unwrap();
                    });
                },
            )
                .add_asset_file("./assets/logo.png".into()),
        )
        .add_story(
            Story::new(
                "segment_canvas",
                story_body! {
                    use egui::{Id, ImageSource};
                    use egui_viscanvas::*;

                    let contents: Vec<Content> = vec![
                        Segment::new(egui::Pos2::new(0.0, 0.0), egui::Pos2::new(100.0, 100.0))
                            .with_stroke_color(egui::Color32::from_rgb(155, 155, 0))
                            .with_stroke_thickness(2.0).into(),
                        Rectangle::new()
                            .with_position(egui::Pos2::new(100.0, 100.0))
                            .with_size(egui::Vec2::new(256.0, 256.0))
                            .with_stroke_color(egui::Color32::from_rgb(155, 155, 0))
                            .with_stroke_thickness(2.0).into(),
                    ];

                    egui::CentralPanel::default().show(ctx, |ui| {
                        vis_canvas(ui, Id::new("canvas"), &contents).unwrap();
                    });
                },
            )
                .add_asset_file("./assets/logo.png".into()),
        )
        .build();

    run_story_book("egui-viscanvas", story_book, NativeOptions::default()).unwrap();
}
