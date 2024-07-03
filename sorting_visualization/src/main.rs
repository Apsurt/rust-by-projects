use eframe::egui;
use egui_plot::{BarChart, Plot, Bar, PlotBounds};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sorting visualizer",
        options,
        Box::new(|_cc| Box::new(SortingVisualizer::default())),
    )
}

struct SortingVisualizer {
    // Add fields here if needed for button functionality
}

impl Default for SortingVisualizer {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for SortingVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Top section for buttons
            ui.vertical_centered(|ui| {
                ui.add_space(30.0); // Add some top margin
                ui.horizontal(|ui| {
                    if ui.button("Shuffle").clicked() {
                        println!("shuffled");
                    }
                    if ui.button("Button 2").clicked() {}
                    if ui.button("Button 3").clicked() {}
                    if ui.button("Button 4").clicked() {}
                    if ui.button("Button 5").clicked() {}
                    if ui.button("Button 6").clicked() {}
                });
            });

            ui.add_space(160.0); // Add space between buttons and plot

            // Bottom section for plot
            let plot = Plot::new("Sorting visualizer")
                .data_aspect(1.0)
                .show_axes([false, false])
                .show_grid([false, false])
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .show_background(false)
                .show_x(false)
                .show_y(false)
                .height(ui.available_height()); // Use all remaining height

            plot.show(ui, |plot_ui| {
                let bars: Vec<Bar> = (-395..=395)
                    .step_by(10)
                    .map(|x| x as f64 * 0.01)
                    .map(|x| {
                        let f = (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt();
                        Bar::new(x, f * 10.0).width(0.095)
                    })
                    .collect();

                let chart = BarChart::new(bars)
                    .color(egui::Color32::WHITE)
                    .name("Sorting visualizer");

                plot_ui.bar_chart(chart);

                // Set custom bounds to remove padding
                plot_ui.set_plot_bounds(PlotBounds::from_min_max([-4.0, 0.0], [4.0, 4.0]));
            });
        });
    }
}