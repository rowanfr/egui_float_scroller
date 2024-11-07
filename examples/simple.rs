use eframe::{egui, NativeOptions};
use egui_float_scroller::FixedScrollbar;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Scrollbar Example",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    side_panel_scroll: f32,
    floating_scroll: f32,
    docked_scroll: f32,
    direct_scroll: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            side_panel_scroll: 0.0,
            floating_scroll: 0.0,
            docked_scroll: 0.0,
            direct_scroll: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Side Panel Scrollbar
        FixedScrollbar::new(&mut self.side_panel_scroll, 20.0)
            .show_in_side_panel(ctx, "Side Panel Scrollbar");

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Scrollbar Types Demo");
            ui.add_space(20.0);

            // Display all scroll positions
            ui.label(format!(
                "Side Panel Scrollbar: {:.2}",
                self.side_panel_scroll
            ));
            ui.label(format!("Floating Scrollbar: {:.2}", self.floating_scroll));
            ui.label(format!("Docked Scrollbar: {:.2}", self.docked_scroll));
            ui.label(format!("Direct UI Scrollbar: {:.2}", self.direct_scroll));
            ui.add_space(20.0);

            // 2. Floating Scrollbar
            let scrollbar = FixedScrollbar::new(&mut self.floating_scroll, 20.0);
            scrollbar.show_floating(ui, egui::pos2(100.0, 200.0));

            // 3. Docked Scrollbar
            let area = egui::Rect::from_min_size(egui::pos2(100.0, 300.0), egui::vec2(200.0, 20.0));
            let scrollbar = FixedScrollbar::new(&mut self.docked_scroll, 20.0);
            scrollbar.show_docked(ui, area);

            // 4. Direct UI Scrollbar
            ui.add_space(40.0);
            ui.add(FixedScrollbar::new(&mut self.direct_scroll, 20.0));
        });
    }
}
