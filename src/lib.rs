pub mod error;

use crate::error::{Result, VisCanvasError};
use egui::epaint::PathShape;
use egui::load::TexturePoll;
use egui::{
    Align2, Color32, Context, Id, ImageSource, Painter, PointerButton, Pos2, Rect, Response,
    Rounding, Sense, SizeHint, Stroke, TextureOptions, Ui, Vec2,
};
use egui::{FontId, Shape};
use num::Zero;

const SCROLL_SPEED: f32 = 1.0;
const ZOOM_SPEED: f32 = 1.0;

#[derive(Debug, Clone, Copy)]
pub enum Origin {
    TopLeft,
    BottomLeft,
}

#[derive(Debug, Clone)]
pub enum Thickness {
    /// Relative to the canvas scale
    Relative(f32),
    /// Absolute
    Absolute(f32),
}

#[derive(Debug, Clone)]
pub enum Content {
    Image(Image),
    Rectangle(Rectangle),
    Circle(Circle),
    Segment(Segment),
    PiecewiseSegment(PiecewiseSegment),
}

impl From<Rectangle> for Content {
    fn from(rect: Rectangle) -> Self {
        Content::Rectangle(rect)
    }
}

#[derive(Debug, Clone)]
pub struct SegmentData {
    pub start: Pos2,
    pub end: Pos2,
}

#[derive(Debug, Clone, Default, Copy)]
pub enum SegmentAccent {
    #[default]
    None,
    Arrow,
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub data: SegmentData,
    pub stroke: Stroke,
    pub accents: (SegmentAccent, SegmentAccent),
}

fn arrow_head_shape(
    point: Pos2,
    back_vector: Vec2,
    thickness: f32,
    fill_color: Color32,
) -> (Shape, Vec2) {
    let v = back_vector.normalized();
    let v1 = Vec2::new(
        v.x * f32::cos(std::f32::consts::FRAC_PI_4) - v.y * f32::sin(std::f32::consts::FRAC_PI_4),
        v.x * f32::sin(std::f32::consts::FRAC_PI_4) + v.y * f32::cos(std::f32::consts::FRAC_PI_4),
    );
    let p1 = point + (v1 * 5.0 * thickness);
    // rotate -45 degree
    let v2 = Vec2::new(
        v.x * f32::cos(-std::f32::consts::FRAC_PI_4) - v.y * f32::sin(-std::f32::consts::FRAC_PI_4),
        v.x * f32::sin(-std::f32::consts::FRAC_PI_4) + v.y * f32::cos(-std::f32::consts::FRAC_PI_4),
    );
    let p2 = point + (v2 * 5.0 * thickness);

    (
        Shape::Path(PathShape::convex_polygon(
            vec![point, p2, p1],
            fill_color,
            Stroke::new(0.0, Color32::default()),
        )),
        v * 3.0 * thickness,
    )
}

impl Segment {
    pub fn new(start: Pos2, end: Pos2) -> Self {
        Self {
            data: SegmentData { start, end },
            stroke: Stroke::new(1.0, Color32::BLACK),
            accents: (SegmentAccent::None, SegmentAccent::None),
        }
    }

    pub fn with_stroke_color(mut self, color: Color32) -> Self {
        self.stroke.color = color;
        self
    }

    pub fn with_stroke_thickness(mut self, thickness: f32) -> Self {
        self.stroke.width = thickness;
        self
    }

    pub fn with_start_accent(mut self, accent: SegmentAccent) -> Self {
        self.accents.0 = accent;
        self
    }

    pub fn with_end_accent(mut self, accent: SegmentAccent) -> Self {
        self.accents.1 = accent;
        self
    }

    pub fn show(
        &self,
        _ui: &mut Ui,
        painter: &mut Painter,
        canvas_state: &VisCanvasStateInner,
    ) -> Result<Option<Response>> {
        let mut start = painter.clip_rect().min
            + (self.data.start.to_vec2() * canvas_state.current_scale_vec() + canvas_state.shift);
        let mut end = painter.clip_rect().min
            + (self.data.end.to_vec2() * canvas_state.current_scale_vec() + canvas_state.shift);

        match self.accents.0 {
            SegmentAccent::Arrow => {
                let (shape, arrow_offset) = arrow_head_shape(
                    start,
                    (end - start).normalized(),
                    self.stroke.width,
                    self.stroke.color,
                );
                painter.add(shape);
                start += arrow_offset;
            }
            SegmentAccent::None => {}
        }

        match self.accents.1 {
            SegmentAccent::Arrow => {
                let (shape, arrow_offset) = arrow_head_shape(
                    end,
                    (start - end).normalized(),
                    self.stroke.width,
                    self.stroke.color,
                );
                painter.add(shape);
                end += arrow_offset;
            }
            SegmentAccent::None => {}
        }

        painter.line_segment([start, end], self.stroke);

        Ok(None)
    }
}

impl From<Segment> for Content {
    fn from(segment: Segment) -> Self {
        Content::Segment(segment)
    }
}

#[derive(Debug, Clone)]
pub struct PiecewiseSegment {
    pub data: Vec<SegmentData>,
    pub stroke: Stroke,
}

impl PiecewiseSegment {
    pub fn show(
        &self,
        _ui: &mut Ui,
        painter: &mut Painter,
        canvas_state: &VisCanvasStateInner,
    ) -> Result<Option<Response>> {
        for segment_data in &self.data {
            let start = painter.clip_rect().min
                + (segment_data.start.to_vec2() * canvas_state.current_scale_vec()
                    + canvas_state.shift);
            let end = painter.clip_rect().min
                + (segment_data.end.to_vec2() * canvas_state.current_scale_vec()
                    + canvas_state.shift);

            painter.line_segment([start, end], self.stroke);
        }
        Ok(None)
    }

    pub fn new(points: Vec<Pos2>) -> Option<Self> {
        if points.len() < 2 {
            return None;
        }

        let mut data = Vec::new();
        for i in 0..points.len() - 1 {
            data.push(SegmentData {
                start: points[i],
                end: points[i + 1],
            });
        }

        Some(Self {
            data,
            stroke: Stroke::new(1.0, Color32::BLACK),
        })
    }

    pub fn with_stroke_color(mut self, color: Color32) -> Self {
        self.stroke.color = color;
        self
    }

    pub fn with_stroke_thickness(mut self, thickness: f32) -> Self {
        self.stroke.width = thickness;
        self
    }
}

impl From<PiecewiseSegment> for Content {
    fn from(piecewise_segment: PiecewiseSegment) -> Self {
        Content::PiecewiseSegment(piecewise_segment)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Circle {
    pub center: Pos2,
    pub radius: f32,
    pub fill_color: Option<Color32>,
    pub stroke: Option<Stroke>,
    pub label: Option<String>,
    pub responsable: bool,
}

impl Circle {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_center(mut self, center: Pos2) -> Self {
        self.center = center;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_fill_color(mut self, fill_color: Color32) -> Self {
        self.fill_color = Some(fill_color);
        self
    }

    pub fn with_stroke_color(mut self, stroke_color: Color32) -> Self {
        if let Some(stroke) = &mut self.stroke {
            stroke.color = stroke_color;
        } else {
            self.stroke = Some(Stroke::new(1.0, stroke_color));
        }
        self
    }

    pub fn with_stroke_thickness(mut self, stroke_thickness: f32) -> Self {
        if let Some(stroke) = &mut self.stroke {
            stroke.width = stroke_thickness;
        } else {
            self.stroke = Some(Stroke::new(stroke_thickness, Color32::BLACK));
        }
        self
    }

    pub fn with_filled(mut self, fill: Color32) -> Self {
        self.fill_color = Some(fill);

        self
    }

    pub fn with_label(mut self, label: impl ToString) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn show(
        &self,
        _ui: &mut Ui,
        painter: &mut Painter,
        canvas_state: &VisCanvasStateInner,
    ) -> Result<Option<Response>> {
        let center = painter.clip_rect().min
            + (self.center.to_vec2() * canvas_state.current_scale_vec() + canvas_state.shift);
        let radius = self.radius * canvas_state.current_scale;

        painter.circle(
            center,
            radius,
            self.fill_color.unwrap_or_default(),
            if let Some(stroke) = &self.stroke {
                *stroke
            } else {
                Stroke::new(0.0, Color32::BLACK)
            },
        );
        if let Some(label) = &self.label {
            let text_rect = painter.text(
                center,
                Align2::CENTER_CENTER,
                label.as_str(),
                FontId::default(),
                Color32::BLACK,
            );
            if let Some(fill_color) = self.fill_color {
                painter.rect_filled(text_rect, 0.0, fill_color);
            }
            let _text_rect = painter.text(
                center,
                Align2::CENTER_CENTER,
                label.as_str(),
                FontId::default(),
                Color32::BLACK,
            );
        }

        Ok(None)
    }
}

impl From<Circle> for Content {
    fn from(circle: Circle) -> Self {
        Content::Circle(circle)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill_color: Option<Color32>,
    pub stroke: Option<Stroke>,
    pub label: Option<String>,
    pub responsable: bool,
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

    pub fn with_fill_color(mut self, fill_color: Color32) -> Self {
        self.fill_color = Some(fill_color);
        self
    }

    pub fn with_stroke_color(mut self, stroke_color: Color32) -> Self {
        if let Some(stroke) = &mut self.stroke {
            stroke.color = stroke_color;
        } else {
            self.stroke = Some(Stroke::new(1.0, stroke_color));
        }
        self
    }

    pub fn with_stroke_thickness(mut self, stroke_thickness: f32) -> Self {
        if let Some(stroke) = &mut self.stroke {
            stroke.width = stroke_thickness;
        } else {
            self.stroke = Some(Stroke::new(stroke_thickness, Color32::BLACK));
        }
        self
    }

    pub fn with_filled(mut self, fill: Color32) -> Self {
        self.fill_color = Some(fill);

        self
    }

    pub fn with_label(mut self, label: impl ToString) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn with_responsable(mut self, responsable: bool) -> Self {
        self.responsable = responsable;
        self
    }

    pub fn show(
        &self,
        ui: &mut Ui,
        painter: &mut Painter,
        canvas_state: &VisCanvasStateInner,
    ) -> Result<Option<Response>> {
        let rect = Rect::from_two_pos(
            painter.clip_rect().min
                + (Vec2::new(self.x, self.y) * canvas_state.current_scale_vec()
                    + canvas_state.shift),
            painter.clip_rect().min
                + (Vec2::new(self.x + self.width, self.y + self.height)
                    * canvas_state.current_scale_vec()
                    + canvas_state.shift),
        );

        painter.rect(
            rect,
            Rounding::default(),
            self.fill_color.unwrap_or_default(),
            if let Some(stroke) = &self.stroke {
                *stroke
            } else {
                Stroke::new(0.0, Color32::BLACK)
            },
        );
        if let Some(label) = &self.label {
            let text_rect = painter.text(
                rect.left_top(),
                Align2::LEFT_BOTTOM,
                label.as_str(),
                FontId::default(),
                Color32::BLACK,
            );
            if let Some(fill_color) = self.fill_color {
                painter.rect_filled(text_rect, 0.0, fill_color);
            }
            let _text_rect = painter.text(
                rect.left_top(),
                Align2::LEFT_BOTTOM,
                label.as_str(),
                FontId::default(),
                Color32::BLACK,
            );
        }

        if self.responsable {
            Ok(Some(ui.allocate_rect(rect, Sense::click())))
        } else {
            Ok(None)
        }
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

pub fn vis_canvas(
    ui: &mut Ui,
    id: Id,
    origin: Origin,
    contents: &[Content],
) -> Result<(Response, VisCanvasState)> {
    let mut state = VisCanvasState::load(ui.ctx(), id, origin);
    let response = state.show_body(ui, contents)?;
    state.store(ui.ctx());
    Ok((response, state))
}

pub struct VisCanvasState {
    pub id: Id,
    inner_state: VisCanvasStateInner,
}

#[derive(Debug, Clone)]
pub struct VisCanvasStateInner {
    origin: Origin,
    current_scale: f32,
    shift: Vec2,
}

impl Default for VisCanvasStateInner {
    fn default() -> Self {
        Self {
            current_scale: 1.0,
            shift: Vec2::ZERO,
            origin: Origin::TopLeft,
        }
    }
}

impl VisCanvasState {
    pub fn screen_to_canvas(&self, screen_pos: Pos2) -> Pos2 {
        assert_ne!(self.inner_state.current_scale, f32::zero());
        ((screen_pos - self.inner_state.shift).to_vec2() / self.inner_state.current_scale_vec())
            .to_pos2()
    }

    pub(crate) fn load(ctx: &Context, id: Id, origin: Origin) -> Self {
        let inner_state = ctx.data_mut(|data| {
            let mut inner = data
                .get_persisted::<VisCanvasStateInner>(id)
                .unwrap_or_default();
            inner.origin = origin;
            inner
        });
        Self { id, inner_state }
    }

    pub(crate) fn store(&self, ctx: &Context) {
        ctx.data_mut(|data| {
            data.insert_persisted(self.id, self.inner_state.clone());
        });
    }

    pub(crate) fn show_body(&mut self, ui: &mut Ui, contents: &[Content]) -> Result<Response> {
        let old_state = self.inner_state.clone();

        let response = ui
            .centered_and_justified(|ui| {
                let (response, mut painter) =
                    ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
                for content in contents {
                    match content {
                        Content::Rectangle(rect) => {
                            rect.show(ui, &mut painter, &self.inner_state)?;
                        }
                        Content::Image(image) => {
                            image.show(ui, &mut painter, &self.inner_state)?;
                        }
                        Content::Segment(segment) => {
                            segment.show(ui, &mut painter, &self.inner_state)?;
                        }
                        Content::PiecewiseSegment(piecewise_segment) => {
                            piecewise_segment.show(ui, &mut painter, &self.inner_state)?;
                        }
                        Content::Circle(circle) => {
                            circle.show(ui, &mut painter, &self.inner_state)?;
                        }
                    }
                }
                Ok::<Response, VisCanvasError>(response)
            })
            .inner?;

        let state = &mut self.inner_state;
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
    pub fn current_scale_vec(&self) -> Vec2 {
        match self.origin {
            Origin::TopLeft => Vec2::new(self.current_scale, self.current_scale),
            Origin::BottomLeft => Vec2::new(self.current_scale, -self.current_scale),
        }
    }

    fn is_valid(&self) -> bool {
        0.0 <= self.current_scale
            && self.current_scale <= 10.0
            && -100000.0 <= self.shift.x
            && self.shift.x <= 100000.0
            && -100000.0 <= self.shift.y
            && self.shift.y <= 100000.0
    }
}
