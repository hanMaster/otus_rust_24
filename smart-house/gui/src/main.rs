#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::toggle::toggle;
use eframe::egui;
use log::{error, info};
use std::error::Error;
use tcp_socket::{SocketConnector, SocketInfo};
use tokio::sync::mpsc;

mod toggle;
#[tokio::main]
async fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Умная розетка",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    is_turned_on: bool,
    power: f64,
    tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
    send_in_progress: bool,
    command_result_msg: String,
}

struct Message {
    is_turned_on: bool,
    power: f64,
    command_result_msg: String,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp::new()
    }
}

impl MyApp {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);
        Self {
            is_turned_on: false,
            power: 0.0,
            tx,
            rx,
            send_in_progress: false,
            command_result_msg: String::new(),
        }
    }

    async fn send_command(
        current_state: &SocketInfo,
        tx: mpsc::Sender<Message>,
    ) -> Result<(), Box<dyn Error>> {
        let cs = current_state.clone();
        let c = tcp_socket::SocketClient::new("127.0.0.1:3456")?;

        let res = if cs.is_turned_on {
            c.turn_off().await
        } else {
            c.turn_on().await
        };
        if res.is_ok() {
            // если удалось включить/выключить значит связь с розеткой есть
            // значит
            let state = c.get_socket_info().await?;
            let msg = Message {
                is_turned_on: state.is_turned_on,
                power: state.power,
                command_result_msg: "".to_string(),
            };
            tx.send(msg).await?;
        }

        if res.is_err() {
            let msg = Message {
                is_turned_on: cs.is_turned_on,
                power: cs.power,
                command_result_msg: "Ошибка связи с розеткой".to_string(),
            };
            tx.send(msg).await?;
        }
        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TCP socket interface");
            ui.separator();
            ui.add(toggle(&mut self.is_turned_on));
            ui.label(format!(
                "Состояние розетки: {}, мощность: {} Ватт",
                if self.is_turned_on {
                    "включена"
                } else {
                    "выключена"
                },
                self.power
            ));
            if ui
                .button(if self.is_turned_on {
                    "Выключить"
                } else {
                    "Включить"
                })
                .clicked()
                && !self.send_in_progress
            {
                self.send_in_progress = true;
                let tx = self.tx.clone();
                let current_state: SocketInfo = SocketInfo {
                    is_turned_on: self.is_turned_on,
                    power: self.power,
                };
                tokio::spawn(async move {
                    if let Err(e) = MyApp::send_command(&current_state, tx).await {
                        error!("Error sending command: {}", e);
                    }
                });
            }

            if let Ok(response) = self.rx.try_recv() {
                info!("Msg: {:?}", response);
                self.is_turned_on = response.is_turned_on;
                self.power = response.power;
                self.send_in_progress = false;
            }
        });
    }
}
