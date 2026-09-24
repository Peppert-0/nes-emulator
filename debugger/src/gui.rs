struct Gui {
    egui_context: egui::Context,
    full_output: egui::FullOutput,
}

#[derive(Debug)]
enum GuiError {}

impl Gui {
    pub fn new(raw_input: egui::RawInput) -> Result<Self, GuiError> {
        let egui_context = egui::Context::default();
        let full_output = egui_context.run_ui(raw_input, |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                ui.label("Hello world!");
                if ui.button("Click me").clicked() {
                    // take some action here
                }
            });
        });

        Ok(Self {
            egui_context,
            full_output,
        })
    }
}
