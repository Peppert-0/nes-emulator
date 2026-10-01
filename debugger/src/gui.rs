use std::collections::HashMap;

use egui::{TextureId, Widget, load::SizedTexture};

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
}
impl Component for PatternTables {
    fn display(&mut self, ui: &mut egui::Ui, resources: &Resources) -> () {
        egui::Window::new("Pattern Tables")
            .auto_sized()
            .show(ui, |ui| {
                if resources.textures.is_empty() {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.allocate_ui_with_layout(
                            egui::Vec2 { x: 256.0, y: 128.0 },
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.label("Drag and drop a ROM file to display its pattern tables.");
                            },
                        );
                    });
                } else {
                    ui.add(egui::Slider::new(&mut self.scale, 1.0..=5.0).text("Scale"));
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        for (texture_name, texture_id) in
                            resources.textures.iter().filter(|(name, id)| {
                                matches!(
                                    name,
                                    TextureName::PatternTable1 | TextureName::PatternTable2
                                )
                            })
                        {
                            egui::Frame::group(ui.style()).show(ui, |ui| {
                                let texture = SizedTexture {
                                    id: *texture_id,
                                    size: egui::Vec2 {
                                        x: 128.0 * self.scale,
                                        y: 128.0 * self.scale,
                                    },
                                };
                                egui::Image::new(texture).ui(ui);
                            });
                        }
                    });
                }
            });
    }
}
