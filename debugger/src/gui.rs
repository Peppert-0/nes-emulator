pub struct Gui {
    pub egui_context: egui::Context,
    pub full_output: egui::FullOutput,
}

#[derive(Debug)]
pub enum GuiError {}

impl Gui {
    pub fn new(raw_input: egui::RawInput) -> Result<Self, GuiError> {
        let egui_context = egui::Context::default();
        let full_output = egui_context.run_ui(raw_input, |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                ui.label("Hello world!");

                let response = ui.button("Click me");

                println!(
                    "hovered={} clicked={} pointer={:?}",
                    response.hovered(),
                    response.clicked(),
                    ui.input(|i| i.pointer.hover_pos()),
                );
            });
        });

        Ok(Self {
            egui_context,
            full_output,
        })
    }
}
