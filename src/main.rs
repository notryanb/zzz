use eframe::{
    egui::{
        CentralPanel,
        Color32,
        CtxRef,
        Label,
        Key,
        TextEdit,
        ScrollArea,
        TopBottomPanel,
        Vec2
    },
    epi::{App, Frame},
    NativeOptions,
    run_native,
};

struct ZzzApp {
    url: String,
    response_content: String,
}

impl App for ZzzApp {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        TopBottomPanel::top("zzz").show(ctx, |ui| {
               ui.label("Welcome to Zzz!");
        });

        CentralPanel::default().show(ctx, |ui| {
            let text_edit_url = TextEdit::singleline(&mut self.url)
                .hint_text("enter url");

            let text_input = ui.add_sized([500.0, 10.0], text_edit_url);

            if text_input.lost_focus() && ui.input().key_pressed(Key::Enter) {
                tracing::info!("{}", &self.url);

                let resp = reqwest::blocking::get(&self.url).expect("failed req");
                let rtext = resp.text();
                match rtext {
                    Ok(content) => self.response_content = content.to_string(),
                    Err(_) => tracing::error!("failed to get response content"),
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

    let zzz_app = ZzzApp { url: "".to_string(), response_content: "...".to_string(), };
    let mut window_options = NativeOptions::default();
    window_options.initial_window_size = Some(Vec2::new(640., 480.));
    run_native(Box::new(zzz_app), window_options);
}
