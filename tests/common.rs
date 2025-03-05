use egui::Id;
use egui_kittest::Harness;
use egui_viscanvas::{vis_canvas, Content, Origin};

/// 指定されたコンテンツでキャンバスを描画し、スナップショットを取得する
#[allow(dead_code)]
pub fn render_and_snapshot(name: &str, contents: &[Content]) {
    let app = move |ctx: &egui::Context| -> () {
        egui::CentralPanel::default().show(ctx, |ui| {
            vis_canvas(ui, Id::new("test_canvas"), Origin::TopLeft, contents).unwrap();
        });
    };

    let mut harness = Harness::new(app);
    harness.run();
    harness.snapshot(name);
}

/// 指定されたコンテンツとズームレベルでキャンバスを描画し、スナップショットを取得する
///
/// 注意: 現在の実装では、ズームレベルを直接設定することはできません。
/// このテストは基本的な描画のみをテストします。
#[allow(dead_code)]
pub fn render_with_zoom_and_snapshot(contents: &[Content], zoom: f32) {
    let app = move |ctx: &egui::Context| -> () {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 基本的な描画のみを行う
            let (_resp, mut state) =
                vis_canvas(ui, Id::new("test_canvas"), Origin::TopLeft, contents).unwrap();

            state.set_zoom(ctx, zoom);

            // ズームレベルの情報を表示
            ui.label(format!("想定ズームレベル: {}", zoom));
        });
    };

    let mut harness = Harness::new(app);
    harness.run();
    harness.snapshot(format!("zoom_{}", zoom).as_str());
}

/// 指定されたコンテンツと原点設定でキャンバスを描画し、スナップショットを取得する
#[allow(dead_code)]
pub fn render_with_origin_and_snapshot(contents: &[Content], origin: Origin) {
    let app = move |ctx: &egui::Context| -> () {
        egui::CentralPanel::default().show(ctx, |ui| {
            vis_canvas(ui, Id::new("test_canvas"), origin, contents).unwrap();
        });
    };

    let mut harness = Harness::new(app);
    harness.run();

    let origin_name = match origin {
        Origin::TopLeft => "top_left",
        Origin::BottomLeft => "bottom_left",
    };

    harness.snapshot(format!("origin_{}", origin_name).as_str());
}
