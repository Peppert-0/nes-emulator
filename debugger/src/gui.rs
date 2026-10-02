use std::collections::HashMap;

use egui::{Align, TextureId, Vec2, Widget, load::SizedTexture};

pub trait Component {
    fn display(&mut self, ui: &mut egui::Ui, resources: &Resources) -> ();
}

pub struct Arguments {
    pub rom_loaded: bool,
}

pub struct Resources {
    pub textures: HashMap<TextureName, TextureId>,
}

#[derive(Hash, PartialEq, Eq)]
pub enum ComponentId {
    PatternTables,
}

#[derive(Hash, PartialEq, Eq)]
pub enum TextureName {
    PatternTable1,
    PatternTable2,
}

struct PatternTables {
    scale: f32,
}

pub struct Gui {
    pub resources: Resources,
    pub components: HashMap<ComponentId, Box<dyn Component>>,
}

impl Gui {
    pub fn new() -> Self {
        let resources = Resources {
            textures: HashMap::new(),
        };
        let components = Self::get_components();

        Self {
            resources,
            components,
        }
    }
    fn get_components() -> HashMap<ComponentId, Box<dyn Component>> {
        let components = [(
            ComponentId::PatternTables,
            Box::new(PatternTables::new()) as Box<dyn Component>,
        )];
        components.into_iter().collect()
    }
    pub fn display_component(&mut self, ui: &mut egui::Ui, id: &ComponentId) -> () {
        let component = self.components.get_mut(id).unwrap();
        component.display(ui, &self.resources);
    }
}

impl PatternTables {
    fn new() -> Self {
        Self { scale: 1.0 }
    }
    fn display_pattern_table(&self, ui: &mut egui::Ui, table: &TextureId, size: f32) -> () {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            let texture = SizedTexture {
                id: *table,
                size: Vec2 { x: 128.0, y: 128.0 },
            };
            egui::Image::new(texture)
                .fit_to_exact_size(Vec2::splat(size))
                .ui(ui);
        });
    }
}
impl Component for PatternTables {
    fn display(&mut self, ui: &mut egui::Ui, resources: &Resources) -> () {
        egui::Window::new("Pattern Tables")
            .resizable(true)
            .default_size(Vec2::new(400.0, 300.0))
            .show(ui, |ui| {
                if resources.textures.is_empty() {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.allocate_ui_with_layout(
                            ui.available_size(),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.label("Drag and drop a ROM file to display its pattern tables.");
                            },
                        );
                    });
                } else {
                    let available = ui.available_size();
                    let table_size = (available.x - 36.0) / 2.0;

                    ui.horizontal(|ui| {
                        self.display_pattern_table(
                            ui,
                            &resources.textures[&TextureName::PatternTable1],
                            table_size,
                        );

                        self.display_pattern_table(
                            ui,
                            &resources.textures[&TextureName::PatternTable2],
                            table_size,
                        );
                    });
                }
            });
    }
}
