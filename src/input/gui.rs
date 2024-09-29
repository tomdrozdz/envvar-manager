use std::sync::mpsc::{channel, Sender};

use anyhow::{anyhow, Context, Result};

use eframe::egui;

pub struct InputApp {
    prompt: String,
    input: String,
    sender: Sender<String>,
}

impl InputApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, prompt: String, sender: Sender<String>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self {
            prompt,
            input: String::new(),
            sender,
        }
    }
}

impl eframe::App for InputApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(&self.prompt);

                let text = ui.text_edit_singleline(&mut self.input);
                let button = ui.button("Submit");

                if (text.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    || button.clicked()
                {
                    self.sender.send(self.input.clone()).unwrap();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

pub fn get_input(prompt: &str) -> Result<String> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    let (sender, receiver) = channel::<String>();
    let res = eframe::run_native(
        "EnvVar Manager: Input window",
        native_options,
        Box::new(|cc| Ok(Box::new(InputApp::new(cc, prompt.to_string(), sender)))),
    );

    let out = receiver
        .recv()
        .context("Failed to receive input from GUI")?;

    match res {
        Ok(()) => {}
        Err(e) => {
            return Err(anyhow!("Error: {}", e));
        }
    }

    Ok(out)
}
