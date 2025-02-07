#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::toggle::toggle;
use eframe::egui;
use log::info;

mod toggle;
#[tokio::main]
async fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "TCP socket interface",
        options,
        Box::new(|_| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    is_on: bool,
    power: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            is_on: false,
            power: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        info!("start update");
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TCP socket interface");
            ui.separator();
            ui.add(toggle(&mut self.is_on));
            ui.label(format!("Socket state '{}', power {}", self.is_on, self.power));
        });
    }
}