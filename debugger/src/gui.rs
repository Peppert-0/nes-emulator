use egui::{Context, FullOutput, RawInput};

pub struct Gui {
    pub egui_context: egui::Context,
    pub full_output: egui::FullOutput,
}

#[derive(Debug)]
pub enum GuiError {}

impl Gui {
    pub fn draw_gui(context: &Context, input: RawInput) -> Result<FullOutput, GuiError> {
        let full_output = context.run_ui(input, |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                ui.label("Hello world!");

                let response = ui.button("Click me");
                if response.clicked() {
                    println!("Click");
                }
            });
        });

        Ok(full_output)
    }
}
