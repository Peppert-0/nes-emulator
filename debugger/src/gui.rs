use std::collections::{HashMap, HashSet};

use egui::{Align, Frame, Margin, TextureId, Vec2, Widget, load::SizedTexture};

pub trait Component {
    fn display(&mut self, ui: &mut egui::Ui, context: &mut GuiContext) -> ();
}

pub struct Arguments {
    pub rom_loaded: bool,
}

pub struct Resources {
    pub textures: HashMap<TextureName, TextureId>,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum ComponentId {
    PatternTables,
    CentralPanel,
    Tray,
}

#[derive(Hash, PartialEq, Eq)]
pub enum TextureName {
    PatternTable1,
    PatternTable2,
}

struct PatternTables {
    open: bool,
    scale: f32,
}
struct CentralPanel {}
struct Tray {
    is_expanded: bool,
}

pub struct Gui {
    pub context: GuiContext,
    pub components: HashMap<ComponentId, Box<dyn Component>>,
}
pub struct GuiContext {
    pub resources: Resources,
    pub windows: HashMap<ComponentId, bool>,
}

impl GuiContext {
    fn new() -> Self {
        let resources = Resources {
            textures: HashMap::new(),
        };
        let windows = Self::get_windows();

        Self { resources, windows }
    }
    fn get_windows() -> HashMap<ComponentId, bool> {
        let windows = [(ComponentId::PatternTables, true)];
        windows.into_iter().collect()
    }
    fn window_open(&mut self, window: ComponentId) -> &mut bool {
        self.windows.get_mut(&window).unwrap()
    }
    fn toggle_window(&mut self, window: ComponentId) -> () {
        let open = self.window_open(window);
        if *open {
            *open = false;
        } else {
            *open = true;
        }
    }
}
impl Gui {
    pub fn new() -> Self {
        let context = GuiContext::new();
        let components = Self::get_components();

        Self {
            context,
            components,
        }
    }
    fn get_components() -> HashMap<ComponentId, Box<dyn Component>> {
        let components = [
            (
                ComponentId::PatternTables,
                Box::new(PatternTables::new()) as Box<dyn Component>,
            ),
            (
                ComponentId::CentralPanel,
                Box::new(CentralPanel::new()) as Box<dyn Component>,
            ),
            (
                ComponentId::Tray,
                Box::new(Tray::new()) as Box<dyn Component>,
            ),
        ];
        components.into_iter().collect()
    }
    pub fn display_component(&mut self, ui: &mut egui::Ui, id: &ComponentId) -> () {
        let component = self.components.get_mut(id).unwrap();
        component.display(ui, &mut self.context);
    }
}

impl PatternTables {
    fn new() -> Self {
        Self {
            open: true,
            scale: 1.0,
        }
    }
    fn display_pattern_table(ui: &mut egui::Ui, table: &TextureId, size: f32) -> () {
        egui::Frame::group(ui.style())
            .inner_margin(0.0)
            .show(ui, |ui| {
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
    fn display(&mut self, ui: &mut egui::Ui, context: &mut GuiContext) -> () {
        egui::Window::new("Pattern Tables")
            .open(
                context
                    .windows
                    .get_mut(&ComponentId::PatternTables)
                    .unwrap(),
            )
            .resizable(true)
            .default_size(Vec2::new(400.0, 235.0))
            .show(ui, |ui| {
                if context.resources.textures.is_empty() {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.allocate_ui_with_layout(
                            ui.available_size(),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.label(
                                    "Drag and drop a ROM file to display its pattern tables here",
                                );
                            },
                        );
                    });
                } else {
                    let available = ui.available_size();
                    let table_size = (available.x - 12.0) / 2.0;

                    ui.horizontal(|ui| {
                        Self::display_pattern_table(
                            ui,
                            &context.resources.textures[&TextureName::PatternTable1],
                            table_size,
                        );

                        Self::display_pattern_table(
                            ui,
                            &context.resources.textures[&TextureName::PatternTable2],
                            table_size,
                        );
                    });
                }
            });
    }
}
impl CentralPanel {
    fn new() -> Self {
        Self {}
    }
}
impl Component for CentralPanel {
    fn display(&mut self, ui: &mut egui::Ui, context: &mut GuiContext) -> () {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Frame::group(ui.style())
                .outer_margin(0.0)
                .show(ui, |ui| {
                    ui.set_min_size(ui.available_size());

                    if context.resources.textures.is_empty() {
                        ui.centered_and_justified(|ui| {
                            ui.label(egui::RichText::new("Drop a ROM here").size(30.0).strong());
                        });
                    }
                });
        });
    }
}
impl Tray {
    fn new() -> Self {
        Self { is_expanded: true }
    }
    fn button(
        &self,
        ui: &mut egui::Ui,
        context: &mut GuiContext,
        window: ComponentId,
        title: String,
    ) -> () {
        let button =
            ui.selectable_label(*context.window_open(window.clone()), format!("{}", title));
        if button.hovered() {
            ui.set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if button.clicked() {
            context.toggle_window(window.clone());
        }
    }
}
impl Component for Tray {
    fn display(&mut self, ui: &mut egui::Ui, context: &mut GuiContext) -> () {
        egui::Panel::right("Tray")
            .show_separator_line(false)
            .frame(Frame::central_panel(ui.style()).outer_margin(0.0))
            .max_size(120.0)
            .show(ui, |ui| {
                Frame::group(ui.style()).inner_margin(0.0).show(ui, |ui| {
                    ui.set_min_size(ui.available_size());
                    ui.vertical_centered(|ui| {
                        ui.add_space(5.0);
                        ui.heading("Tools");
                        ui.separator();

                        self.button(
                            ui,
                            context,
                            ComponentId::PatternTables,
                            "Pattern Tables".into(),
                        );
                    })
                })
            });
    }
}
