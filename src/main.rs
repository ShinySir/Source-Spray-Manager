use eframe::*;
use egui::CentralPanel;

struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    run_native("Source Spray Manager",
    NativeOptions::default(),
    AppCreator::Box::new(| cc: &CreationContext<'_>|{
        Box::new(MyApp {})
    }));
}
//test