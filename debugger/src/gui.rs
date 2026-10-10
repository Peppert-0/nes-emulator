use core::{
    command::{self, Command},
    console::ConsoleView,
    cpu::InstructionView,
};
use std::collections::{HashMap, HashSet};

use builder::Builder;
use egui::{
    Align, Color32, Frame, Layout, Margin, RichText, TextureId, Vec2, Widget, load::SizedTexture,
};

pub trait Component {
    fn display(&mut self, ui: &mut egui::Ui, context: &mut GuiContext) -> ();
}

pub struct Resources {
    pub textures: HashMap<TextureName, TextureId>,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum ComponentId {
    PatternTables,
    CpuViewWindow,
    ControlsWindow,
    InstructionViewWindow,
    CentralPanel,
    Tray,
}

#[derive(Hash, PartialEq, Eq)]
pub enum TextureName {
    PatternTable1,
    PatternTable2,
}

#[derive(Builder)]
struct PatternTables {}
#[derive(Builder)]
struct CpuViewWindow {}
#[derive(Builder)]
pub struct ControlsWindow {}
#[derive(Builder)]
pub struct InstructionViewWindow {}
#[derive(Builder)]
struct CentralPanel {}
#[derive(Builder)]
struct Tray {}

pub struct Gui {
    pub context: GuiContext,
    pub components: HashMap<ComponentId, Box<dyn Component>>,
}
pub struct GuiContext {
    pub resources: Resources,
    pub windows: HashMap<ComponentId, bool>,
    pub emulator: Option<ConsoleView>,
    pub emulator_running: bool,
    pub commands: Vec<Box<dyn Command>>,
}

impl GuiContext {
    fn new() -> Self {
        let resources = Resources {
            textures: HashMap::new(),
        };
        let windows = Self::get_windows();
        let emulator = None;
        let emulator_running = false;
        let commands = Vec::new();

        Self {
            resources,
            windows,
            emulator,
            emulator_running,
            commands,
        }
    }
    fn get_windows() -> HashMap<ComponentId, bool> {
        let windows = [
            (ComponentId::PatternTables, false),
            (ComponentId::CpuViewWindow, true),
            (ComponentId::ControlsWindow, true),
            (ComponentId::InstructionViewWindow, true),
        ];
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
fn toggle(value: &mut bool) {
    if *value {
        *value = false;
    } else {
        *value = true;
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
                ComponentId::CpuViewWindow,
                Box::new(CpuViewWindow::new()) as Box<dyn Component>,
            ),
            (
                ComponentId::ControlsWindow,
                Box::new(ControlsWindow::new()) as Box<dyn Component>,
            ),
            (
                ComponentId::InstructionViewWindow,
                Box::new(InstructionViewWindow::new()) as Box<dyn Component>,
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
impl Component for CpuViewWindow {
    fn display(&mut self, ui: &mut egui::Ui, context: &mut GuiContext) -> () {
        egui::Window::new("CPU View")
            .open(
                context
                    .windows
                    .get_mut(&ComponentId::CpuViewWindow)
                    .unwrap(),
            )
            .resizable(false)
            .fixed_size(Vec2::new(280.0, 0.0))
            .show(ui, |ui| {
                if let Some(emulator) = &context.emulator {
                    let registers = [
                        (
                            "Program Counter:",
                            format!("0x{:04X}", emulator.cpu_view.registers.pc),
                        ),
                        (
                            "Stack Pointer:",
                            format!("0x{:02X}", emulator.cpu_view.registers.sp),
                        ),
                        (
                            "Accumulator:",
                            format!("0x{:02X}", emulator.cpu_view.registers.a),
                        ),
                        (
                            "X Register:",
                            format!("0x{:02X}", emulator.cpu_view.registers.x),
                        ),
                        (
                            "Y Register:",
                            format!("0x{:02X}", emulator.cpu_view.registers.y),
                        ),
                        ("Flags:", format!("0x{:02X}", emulator.cpu_view.registers.p)),
                    ];

                    let row_width = ui.available_width();
                    let row_height = 20.0;
                    let name_width = 120.0;

                    for (i, (name, value)) in registers.iter().enumerate() {
                        let alternate = i % 2 == 1;

                        let (row_rect, _) = ui.allocate_exact_size(
                            egui::vec2(row_width, row_height),
                            egui::Sense::hover(),
                        );

                        let white = Color32::from_hex("#ebdbb2").unwrap();
                        let grey = Color32::from_hex("#292929").unwrap();
                        let black = Color32::from_hex("#3c3836").unwrap();

                        if alternate {
                            ui.painter().rect_filled(row_rect, 0.0, grey);
                        }

                        let text_color = if alternate {
                            ui.visuals().text_color()
                        } else {
                            ui.visuals().text_color()
                        };

                        // Register name
                        ui.put(
                            egui::Rect::from_min_size(
                                row_rect.min + egui::vec2(4.0, 0.0),
                                egui::vec2(name_width, row_height),
                            ),
                            egui::Label::new(egui::RichText::new(*name).color(text_color)),
                        );

                        // Register value
                        ui.put(
                            egui::Rect::from_min_size(
                                egui::pos2(row_rect.max.x - 4.0 - 60.0, row_rect.min.y),
                                egui::vec2(60.0, row_height),
                            ),
                            egui::Label::new(egui::RichText::new(value).color(text_color))
                                .halign(egui::Align::RIGHT),
                        );
                    }
                }
            });
    }
}
impl Component for ControlsWindow {
    fn display(&mut self, ui: &mut egui::Ui, context: &mut GuiContext) -> () {
        egui::Window::new("Controls")
            .open(
                context
                    .windows
                    .get_mut(&ComponentId::ControlsWindow)
                    .unwrap(),
            )
            .auto_sized()
            .show(ui, |ui| {
                if ui.button("Step").clicked() {
                    context.commands.push(Box::new(command::Step::new()));
                }
                if ui.button("Reset").clicked() {
                    context.commands.push(Box::new(command::Reset::new()));
                }
                if ui.button("Run").clicked() {
                    toggle(&mut context.emulator_running);
                }
            });
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
                    };
                    if let Some(emulator) = &context.emulator {
                        ui.vertical_centered(|ui| {
                            ui.heading(format!("{}", emulator.cartridge_view.id));
                        });
                    };
                });
        });
    }
}
impl Tray {
    fn button(
        &self,
        ui: &mut egui::Ui,
        context: &mut GuiContext,
        window: ComponentId,
        title: String,
    ) -> () {
        let button = ui.add_sized(
            [ui.available_width() - 6.0, 0.0],
            egui::Button::selectable(*context.window_open(window.clone()), title),
        );
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
                        self.button(ui, context, ComponentId::CpuViewWindow, "CPU View".into());
                        self.button(ui, context, ComponentId::ControlsWindow, "Controls".into());
                        self.button(
                            ui,
                            context,
                            ComponentId::InstructionViewWindow,
                            "Instructions".into(),
                        );
                    })
                })
            });
    }
}
impl InstructionViewWindow {
    fn cell(ui: &mut egui::Ui, width: f32, text: RichText) {
        ui.allocate_ui_with_layout(
            egui::vec2(width, ui.spacing().interact_size.y),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.set_min_width(width); // reserve the full column width
                ui.label(text);
            },
        );
    }
    fn display_instruction(
        ui: &mut egui::Ui,
        instruction: &InstructionView,
        (address_width, bytes_width, instruction_width): (f32, f32, f32),
    ) {
        Self::cell(
            ui,
            address_width,
            RichText::new(format!("${:04X}", instruction.address))
                .monospace()
                .color(Color32::from_hex("#458588").unwrap()),
        );
        let mut bytes = String::new();
        for byte in &instruction.bytes {
            let string = format!("{:02X} ", byte);
            bytes.push_str(&string);
        }
        let bytes = bytes.trim_end();
        Self::cell(
            ui,
            bytes_width,
            RichText::new(bytes)
                .monospace()
                .color(Color32::from_hex("#689d6a").unwrap()),
        );
        let pink = Color32::from_hex("#d3869b").unwrap();
        let pink2 = Color32::from_hex("#b16286").unwrap();
        let text = match &instruction.operand {
            Some(operand) => {
                format!("{} {}", instruction.mnemonic, operand)
            }
            None => instruction.mnemonic.to_string(),
        };
        Self::cell(
            ui,
            instruction_width,
            RichText::new(text).monospace().color(pink2),
        );
        ui.end_row();
    }
}
impl Component for InstructionViewWindow {
    fn display(&mut self, ui: &mut egui::Ui, context: &mut GuiContext) -> () {
        egui::Window::new("Instructions")
            .open(
                context
                    .windows
                    .get_mut(&ComponentId::InstructionViewWindow)
                    .unwrap(),
            )
            .auto_sized()
            .resizable([false, true])
            .show(ui, |ui| {
                let address_width = 65.0;
                let bytes_width = 100.0;
                let instruction_width = 120.0;
                let widths = (address_width, bytes_width, instruction_width);

                egui::Grid::new("header").num_columns(3).show(ui, |ui| {
                    Self::cell(ui, address_width, RichText::new("ADDR"));
                    Self::cell(ui, bytes_width, RichText::new("BYTES"));
                    Self::cell(ui, instruction_width, RichText::new("INSTRUCTION"));
                    ui.end_row();
                });
                let rect = ui.min_rect();
                let y = ui.cursor().top();

                ui.painter().line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 73, 69)),
                );
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        let mut index: usize = 0;
                        if let Some(emulator) = &context.emulator
                            && !emulator.cpu_view.instructions.is_empty()
                        {
                            egui::Grid::new("cpu_trace")
                                .striped(true)
                                .num_columns(3)
                                .show(ui, |ui| {
                                    for (_i, instruction) in
                                        emulator.cpu_view.instructions.iter().enumerate().filter(
                                            |(i, _instruction)| {
                                                *i != emulator.cpu_view.instructions.len() - 1
                                            },
                                        )
                                    {
                                        Self::display_instruction(ui, instruction, widths);
                                    }
                                });
                            ui.label("NEXT INSTRUCTION:");
                            let next_instruction = &emulator.cpu_view.instructions
                                [emulator.cpu_view.instructions.len() - 1];
                            egui::Grid::new("cpu_trace")
                                .striped(true)
                                .num_columns(3)
                                .show(ui, |ui| {
                                    Self::display_instruction(ui, next_instruction, widths);
                                });
                        }
                    });
            });
    }
}
