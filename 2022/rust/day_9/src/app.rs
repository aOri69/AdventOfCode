use std::collections::VecDeque;

use egui::plot::Points;

use crate::{
    command::{Command, Direction},
    rope::{Position, Rope},
};

pub struct RopeApp {
    commands: VecDeque<Command>,
    rope: Rope,
    timer: std::time::Instant,
    update_interval: std::time::Duration,
}

impl Default for RopeApp {
    fn default() -> Self {
        let commands = Command::get_commands(
            std::fs::read_to_string("input.txt")
                .unwrap_or_default() // will get empty string on Err
                .as_str(),
        )
        .unwrap_or_default()
        .into();
        Self {
            commands,
            rope: Rope::new(10),
            timer: std::time::Instant::now(),
            update_interval: std::time::Duration::from_millis(250u64),
        }
    }
}

impl RopeApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn plot(&self, ui: &mut egui::Ui) -> egui::Response {
        use egui::plot::PlotPoints;

        let plot_points: PlotPoints = self
            .rope
            .tail_visits()
            .iter()
            .map(|tail_visit| [tail_visit.x as f64, tail_visit.y as f64])
            .collect();

        let tail_points = Points::new(plot_points)
            .radius(5.0)
            .shape(egui::plot::MarkerShape::Square)
            .color(egui::Color32::GREEN)
            .name("Tail");

        let plot_points: PlotPoints = PlotPoints::new(vec![[
            self.rope.head().unwrap_or(&Position::default()).x as f64,
            self.rope.head().unwrap_or(&Position::default()).y as f64,
        ]]);

        let head_point = Points::new(plot_points)
            .radius(5.0)
            .shape(egui::plot::MarkerShape::Diamond)
            .color(egui::Color32::RED)
            .name("Head");

        egui::plot::Plot::new("example_plot")
            .center_x_axis(true)
            .center_y_axis(true)
            .auto_bounds_x()
            .auto_bounds_y()
            .data_aspect(1.0)
            .legend(egui::plot::Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.points(tail_points);
                plot_ui.points(head_point);
            })
            .response
    }

    fn timed_command_process(&mut self) {
        // Update by timer
        if self.timer.elapsed() >= self.update_interval {
            if let Some(cmd) = self.commands.pop_front() {
                self.rope.process_command(cmd);
            }
            self.timer = std::time::Instant::now();
        }
    }

    fn integer_edit_field(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut tmp_value = format!("{}", self.update_interval.as_secs_f64());
        let res = ui.text_edit_singleline(&mut tmp_value);
        if let Ok(result) = tmp_value.parse() {
            self.update_interval = std::time::Duration::from_secs_f64(result);
        }
        res
    }
}

impl eframe::App for RopeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto update
        ctx.request_repaint_after(self.update_interval);
        // Update by timer
        self.timed_command_process();

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Commands:");

            if ui.button("Process command").clicked() {
                if let Some(cmd) = self.commands.pop_front() {
                    self.rope.process_command(cmd);
                }
            }
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.commands.iter().for_each(|cmd| {
                    let arrow = match cmd.direction() {
                        Direction::Up => "⬆",
                        Direction::Down => "⬇",
                        Direction::Right => "➡",
                        Direction::Left => "⬅",
                    };
                    ui.label(arrow.repeat(cmd.steps() as usize));
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rope plot");
            ui.separator();
            ui.horizontal(|ui| {
                self.integer_edit_field(ui);
                ui.label(self.timer.elapsed().as_millis().to_string());
            });
            ui.separator();
            ui.label(&format!(
                "Total tail visits:{}",
                self.rope.tail_visits_count()
            ));
            self.plot(ui);
            egui::warn_if_debug_build(ui);
        });
    }
}

pub fn run_gui() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rope simulation",
        native_options,
        Box::new(|cc| Box::new(RopeApp::new(cc))),
    )
}
