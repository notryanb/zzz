use eframe::{
    egui::{CentralPanel, Color32, CtxRef, Key, Label, ScrollArea, TextEdit, TopBottomPanel, Vec2},
    epi::{App, Frame},
    run_native, NativeOptions,
};

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

struct ZzzApp {
    url: String,
    response_content: String,
    sender: Option<Sender<String>>,
    receiver: Option<Receiver<String>>,
}

impl App for ZzzApp {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        TopBottomPanel::top("zzz").show(ctx, |ui| {
            ui.label("Welcome to Zzz!");
        });

        CentralPanel::default().show(ctx, |ui| {
            let text_edit_url = TextEdit::singleline(&mut self.url).hint_text("enter url");
            let text_input = ui.add_sized([500.0, 10.0], text_edit_url);

            if text_input.lost_focus() && ui.input().key_pressed(Key::Enter) {
                let url = self.url.to_string();
                let tx = self.sender.as_ref().unwrap().clone();

                thread::spawn(move || {
                    tracing::info!("{}", url);

                    let resp = reqwest::blocking::get(url);
                    match resp {
                        Ok(r) => {
                            let rtext = r.text();
                            match rtext {
                                Ok(content) => {
                                    tracing::info!("{}", &content);
                                    tx.send(content.to_string()).expect("Failed to send")
                                }
                                Err(_) => tracing::error!("failed to get response content"),
                            }
                        }
                        Err(e) => tx.send(e.to_string()).expect("failed to send"),
                    }
                });
            }

            if let Some(rx) = &self.receiver {
                match rx.try_recv() {
                    Ok(msg) => self.response_content = msg.to_string(),
                    Err(_) => ()//tracing::warn!("Error receiving message")
                }
            }

            ui.label("Result");
            ScrollArea::vertical().show(ui, |ui| {
                let response_label = Label::new(&self.response_content)
                    .monospace()
                    .text_color(Color32::RED);

                ui.add(response_label);
            });
        });
    }

    fn name(&self) -> &str {
        "Zzz"
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("App booted");

    let (tx, rx) = channel();

    let zzz_app = ZzzApp {
        url: "".to_string(),
        response_content: "...".to_string(),
        sender: Some(tx),
        receiver: Some(rx),
    };

    let mut window_options = NativeOptions::default();
    window_options.initial_window_size = Some(Vec2::new(640., 480.));
    run_native(Box::new(zzz_app), window_options);
}
