use eframe::egui::{self, Context, Response, Sense, Ui, Widget};

/// A fixed-size scrollbar that mutates an f32 value between 0.0 and 1.0
pub struct FixedScrollbar<'a> {
    value: &'a mut f32,
    width: f32,
    handle_height: f32,
    scroll_sensitivity: f32,
    scroll_smoothing: bool,
}

impl<'a> FixedScrollbar<'a> {
    pub fn new(value: &'a mut f32, width: f32) -> Self {
        *value = value.clamp(0.0, 1.0);
        Self {
            value,
            width,
            handle_height: 50.0,
            scroll_sensitivity: 0.1,
            scroll_smoothing: true,
        }
    }

    pub fn scroll_sensitivity(mut self, sensitivity: f32) -> Self {
        self.scroll_sensitivity = sensitivity;
        self
    }

    pub fn scroll_smoothing(mut self, scroll_smoothing: bool) -> Self {
        self.scroll_smoothing = scroll_smoothing;
        self
    }

    pub fn handle_height(mut self, handle_height: f32) -> Self {
        self.handle_height = handle_height;
        self
    }

    /// Shows the scrollbar in a right-aligned side panel with sensible defaults
    pub fn show_in_side_panel(self, ctx: &Context, title: impl Into<String>) {
        egui::SidePanel::right(title.into())
            .resizable(false)
            .max_width(self.width)
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                ui.add(self);
            });
    }

    /// Shows the scrollbar as a floating element with automatic positioning
    pub fn show_floating(self, ui: &mut Ui, pos: egui::Pos2) {
        let area = egui::Area::new("floating_scrollbar".into())
            .movable(false)
            .fixed_pos(pos);

        area.show(ui.ctx(), |ui| {
            ui.add(self);
        });
    }

    /// Shows the scrollbar docked to the right of a specified area
    pub fn show_docked(self, ui: &mut Ui, area: egui::Rect) {
        let scroll_rect = egui::Rect::from_min_size(
            egui::pos2(area.max.x - self.width, area.min.y),
            egui::vec2(self.width, area.height()),
        );

        let area = egui::Area::new("docked_scrollbar".into())
            .movable(false)
            .fixed_pos(scroll_rect.min);

        area.show(ui.ctx(), |ui| {
            ui.add(self);
        });
    }
}

impl Widget for FixedScrollbar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let available_height = ui.available_height();

        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(self.width, available_height),
            Sense::click_and_drag(),
        );

        if response.dragged() {
            if let Some(pointer) = response.interact_pointer_pos() {
                let normalized = (pointer.y - rect.min.y) / rect.height();
                *self.value = normalized.clamp(0.0, 1.0);
            }
        }

        let scroll_delta = if self.scroll_smoothing {
            ui.input(|i| i.smooth_scroll_delta.y)
        } else {
            ui.input(|i| i.raw_scroll_delta.y)
        };

        if scroll_delta != 0.0 {
            *self.value = (*self.value
                - (scroll_delta * self.scroll_sensitivity / available_height))
                .clamp(0.0, 1.0);
        }

        // Draw the background
        ui.painter().rect_filled(
            egui::Rect::from_min_size(rect.min, egui::vec2(self.width, rect.height())),
            0.0,
            ui.style().visuals.extreme_bg_color,
        );

        // Draw the handle with configurable height
        let handle_height = (available_height * 0.2).min(self.handle_height);
        let handle_y = rect.min.y + (rect.height() - handle_height) * *self.value;

        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(rect.min.x, handle_y),
                egui::vec2(self.width, handle_height),
            ),
            0.0,
            ui.style().visuals.widgets.active.bg_fill,
        );

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::Context;
    use egui::{LayerId, UiBuilder};

    /// Helper function to create a test context with proper initialization
    fn test_context() -> Context {
        let ctx = Context::default();
        // Initialize the context with a frame
        ctx.begin_pass(egui::RawInput::default());
        ctx
    }

    /// Helper function to create a test UI
    fn test_ui(ctx: &Context) -> Ui {
        let mut ui = Ui::new(
            ctx.clone(),
            LayerId::new(egui::Order::Foreground, "test".into()),
            egui::Id::new("test"),
            UiBuilder::default(),
        );
        ui.set_style(egui::Style::default());
        ui
    }

    #[test]
    fn test_scrollbar_creation() {
        let mut value = 0.5;
        let scrollbar = FixedScrollbar::new(&mut value, 20.0);
        assert_eq!(scrollbar.width, 20.0);
        assert_eq!(*scrollbar.value, 0.5);
    }

    #[test]
    fn test_scrollbar_builder() {
        let mut value = 0.5;
        let scrollbar = FixedScrollbar::new(&mut value, 20.0)
            .scroll_sensitivity(0.2)
            .scroll_smoothing(false)
            .handle_height(30.0);

        assert_eq!(scrollbar.scroll_sensitivity, 0.2);
        assert!(!scrollbar.scroll_smoothing);
        assert_eq!(scrollbar.handle_height, 30.0);
    }

    #[test]
    fn test_scrollbar_clamping() {
        let ctx = test_context();
        let mut ui = test_ui(&ctx);

        let mut value = 2.0; // Start with an out-of-bounds value
        let scrollbar = FixedScrollbar::new(&mut value, 20.0);
        ui.add(scrollbar);

        // Value should be clamped to 1.0
        assert!(value <= 1.0);

        let mut value = -1.0;
        let scrollbar = FixedScrollbar::new(&mut value, 20.0);
        ui.add(scrollbar);

        // Value should be clamped to 0.0
        assert!(value >= 0.0);
    }

    #[test]
    fn test_side_panel_creation() {
        let ctx = test_context();

        let mut value = 0.5;
        let scrollbar = FixedScrollbar::new(&mut value, 20.0);

        // This should not panic
        scrollbar.show_in_side_panel(&ctx, "test_panel");
    }

    #[test]
    fn test_floating_creation() {
        let ctx = test_context();
        let mut ui = test_ui(&ctx);

        let mut value = 0.5;
        let scrollbar = FixedScrollbar::new(&mut value, 20.0);

        // This should not panic
        scrollbar.show_floating(&mut ui, egui::pos2(100.0, 100.0));
    }

    #[test]
    fn test_docked_creation() {
        let ctx = test_context();
        let mut ui = test_ui(&ctx);

        let mut value = 0.5;
        let scrollbar = FixedScrollbar::new(&mut value, 20.0);

        let area = ui.available_rect_before_wrap();
        // This should not panic
        scrollbar.show_docked(&mut ui, area);
    }
}
