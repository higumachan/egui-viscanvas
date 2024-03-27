pub mod error;

use crate::error::{Result, VisCanvasError};
use egui::load::{ImageLoader, TexturePoll};
use egui::style::default_text_styles;
use egui::FontId;
use egui::{
    Align2, Color32, Context, Id, ImageData, ImageSource, Layout, Painter, PointerButton, Pos2,
    Rect, Response, Rounding, Sense, SizeHint, Stroke, TextureId, TextureOptions, Ui, Vec2,
};

const SCROLL_SPEED: f32 = 1.0;
const ZOOM_SPEED: f32 = 1.0;

#[derive(Debug, Clone)]
pub enum Thickness {
    /// Relative to the canvas scale
    Relative(f32),
    /// Absolute
    Absolute(f32),
}

pub enum Content {
    Image(Image),
    Rectangle(Rectangle),
    Segment,
}

impl From<Rectangle> for Content {
    fn from(rect: Rectangle) -> Self {
        Content::Rectangle(rect)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill_color: Option<Color32>,
    pub stroke_color: Option<Color32>,
    pub stroke_thickness: f32,
    pub filled: bool,
    pub label: Option<String>,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_position(mut self, pos: Pos2) -> Self {
        self.x = pos.x;
        self.y = pos.y;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.width = size.x;
        self.height = size.y;
        self
    }

    pub fn with_fill_color(mut self, fill_color: egui::Color32) -> Self {
        self.fill_color = Some(fill_color);
        self
    }

    pub fn with_stroke_color(mut self, stroke_color: egui::Color32) -> Self {
        self.stroke_color = Some(stroke_color);
        self
    }

    pub fn with_stroke_thickness(mut self, stroke_thickness: f32) -> Self {
        self.stroke_thickness = stroke_thickness;
        self
    }

    pub fn with_filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    pub fn with_label(mut self, label: impl ToString) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn show(
        &self,
        ui: &mut Ui,
        painter: &mut Painter,
        canvas_state: &VisCanvasStateInner,
    ) -> Result<Option<Response>> {
        let rect = Rect::from_min_max(
            painter.clip_rect().min
                + (Vec2::new(self.x, self.y) * canvas_state.current_scale + canvas_state.shift),
            painter.clip_rect().min
                + (Vec2::new(self.x + self.width, self.y + self.height)
                    * canvas_state.current_scale
                    + canvas_state.shift),
        );
        painter.rect(
            rect,
            Rounding::default(),
            self.fill_color.unwrap_or_default(),
            Stroke::new(self.stroke_thickness, self.stroke_color.unwrap_or_default()),
        );
        if let Some(label) = &self.label {
            let text_rect = painter.text(
                rect.left_top(),
                Align2::LEFT_BOTTOM,
                label.as_str(),
                FontId::default(),
                Color32::BLACK,
            );
            painter.rect_filled(text_rect, 0.0, self.stroke_color.unwrap_or_default());
            let _text_rect = painter.text(
                rect.left_top(),
                Align2::LEFT_BOTTOM,
                label.as_str(),
                FontId::default(),
                Color32::BLACK,
            );
        }

        // TODO: non fill response
        // TODO: drag response
        Ok(Some(ui.allocate_rect(rect, Sense::click())))
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    image_source: ImageSource<'static>,
}

impl From<Image> for Content {
    fn from(value: Image) -> Self {
        Content::Image(value)
    }
}

impl Image {
    pub fn new(image_source: ImageSource<'static>) -> Self {
        Self { image_source }
    }

    pub fn show(
        &self,
        ui: &mut Ui,
        painter: &mut Painter,
        canvas_state: &VisCanvasStateInner,
    ) -> Result<Option<Response>> {
        let texture = self.image_source.clone().load(
            ui.ctx(),
            TextureOptions::default(),
            SizeHint::Scale(1.0.into()),
        )?;

        if let TexturePoll::Ready { texture } = texture {
            painter.image(
                texture.id,
                Rect::from_min_size(
                    painter.clip_rect().min
                        + (Vec2::new(0.0, 0.0) * canvas_state.current_scale + canvas_state.shift),
                    texture.size * canvas_state.current_scale,
                ),
                Rect::from_min_size(Pos2::ZERO, Vec2::new(1.0, 1.0)),
                Color32::WHITE,
            );
            Ok(None)
        } else {
            Ok(None)
        }
    }
}

pub fn vis_canvas(ui: &mut Ui, id: Id, contents: &[Content]) -> Result<Response> {
    let mut state = VisCanvasState::load(ui.ctx(), id);
    let response = state.show_body(ui, contents)?;
    state.store(ui.ctx());
    Ok(response)
}

pub struct VisCanvasState {
    pub id: Id,
    inner_state: VisCanvasStateInner,
}

#[derive(Debug, Clone)]
struct VisCanvasStateInner {
    current_scale: f32,
    shift: Vec2,
}

impl Default for VisCanvasStateInner {
    fn default() -> Self {
        Self {
            current_scale: 1.0,
            shift: Vec2::ZERO,
        }
    }
}

impl VisCanvasState {
    pub fn load(ctx: &Context, id: Id) -> Self {
        let inner_state = ctx.data_mut(|data| {
            data.get_persisted::<VisCanvasStateInner>(id)
                .unwrap_or_default()
        });
        Self { id, inner_state }
    }

    pub fn store(&self, ctx: &Context) {
        ctx.data_mut(|data| {
            data.insert_persisted(self.id, self.inner_state.clone());
        });
    }

    pub fn show_body(&mut self, ui: &mut Ui, contents: &[Content]) -> Result<Response> {
        let old_state = self.inner_state.clone();

        let response = ui
            .centered_and_justified(|ui| {
                let (response, mut painter) =
                    ui.allocate_painter(ui.available_size(), Sense::drag());
                for content in contents {
                    match content {
                        Content::Rectangle(rect) => {
                            rect.show(ui, &mut painter, &self.inner_state)?;
                        }
                        Content::Image(image) => {
                            image.show(ui, &mut painter, &self.inner_state)?;
                        }
                        _ => {}
                    }
                }
                Ok::<Response, VisCanvasError>(response)
            })
            .inner?;

        let mut state = &mut self.inner_state;
        if response.dragged_by(PointerButton::Middle) {
            state.shift += response.drag_delta();
        }

        if let Some(hover_pos) = response.hover_pos() {
            let hover_pos = hover_pos - response.rect.min;
            ui.input(|input| {
                // スクロール関係
                {
                    let dy = input.raw_scroll_delta.y;
                    let dx = input.raw_scroll_delta.x;
                    state.shift += egui::vec2(dx, dy) * SCROLL_SPEED;
                }
                // ズーム関係
                {
                    // https://chat.openai.com/share/e/c46c2795-a9e4-4f23-b04c-fa0b0e8ab818
                    let scale = input.zoom_delta() * ZOOM_SPEED;
                    let pos = hover_pos;
                    state.current_scale *= scale;
                    state.shift = state.shift * scale
                        + egui::vec2(-scale * pos.x + pos.x, -scale * pos.y + pos.y);
                }
            });
        }

        if !state.is_valid() {
            *state = old_state;
        }

        Ok(response)
    }
}

impl VisCanvasStateInner {
    fn is_valid(&self) -> bool {
        0.0 <= self.current_scale
            && self.current_scale <= 10.0
            && -100000.0 <= self.shift.x
            && self.shift.x <= 100000.0
            && -100000.0 <= self.shift.y
            && self.shift.y <= 100000.0
    }
}
