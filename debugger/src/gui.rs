use core::{
    command::{self, Command},
    console::ConsoleView,
    cpu::InstructionView,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::format,
    hash::Hash,
};

use builder::Builder;
use egui::{
    Align, AsIdSalt, Color32, Frame, IdSalt, Layout, Margin, RichText, TextureId, Vec2, Widget,
    load::SizedTexture,
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
    PpuViewWindow,
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
struct PpuViewWindow {}
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
            (ComponentId::PpuViewWindow, true),
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
                ComponentId::PpuViewWindow,
                Box::new(PpuViewWindow::new()) as Box<dyn Component>,
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
    fn display(&mut self, ui: &mut egui::Ui, context: &mut GuiContext) {
        egui::Window::new("CPU View")
            .open(
                context
                    .windows
                    .get_mut(&ComponentId::CpuViewWindow)
                    .unwrap(),
            )
            .resizable(false)
            .fixed_size(Vec2::new(200.0, 0.0))
            .show(ui, |ui| {
                if let Some(emulator) = &context.emulator {
                    let registers = [
                        (
                            "Program Counter:".to_string(),
                            format!("0x{:04X}", emulator.cpu_view.registers.pc),
                        ),
                        (
                            "Stack Pointer:".to_string(),
                            format!("0x{:02X}", emulator.cpu_view.registers.sp),
                        ),
                        (
                            "Accumulator:".to_string(),
                            format!("0x{:02X}", emulator.cpu_view.registers.a),
                        ),
                        (
                            "X Register:".to_string(),
                            format!("0x{:02X}", emulator.cpu_view.registers.x),
                        ),
                        (
                            "Y Register:".to_string(),
                            format!("0x{:02X}", emulator.cpu_view.registers.y),
                        ),
                    ];

                    let flags = [
                        (
                            "Flags:".to_string(),
                            format!("0x{:02X}", emulator.cpu_view.registers.p),
                        ),
                        (
                            "Carry:".to_string(),
                            format!("{}", emulator.cpu_view.flags.carry),
                        ),
                        (
                            "Zero:".to_string(),
                            format!("{}", emulator.cpu_view.flags.carry),
                        ),
                        (
                            "Interrupt\nDisable:".to_string(),
                            format!("{}", emulator.cpu_view.flags.interrupt_disable),
                        ),
                        (
                            "Decimal:".to_string(),
                            format!("{}", emulator.cpu_view.flags.decimal),
                        ),
                        (
                            "Overflow:".to_string(),
                            format!("{}", emulator.cpu_view.flags.overflow),
                        ),
                        (
                            "Negative:".to_string(),
                            format!("{}", emulator.cpu_view.flags.negative),
                        ),
                    ];

                    key_value_grid(ui, "registers", &registers, false);
                    ui.add_space(2.5);
                    collapsible_key_value_grid(ui, "flags", &flags, true);
                }
            });
    }
}
fn key_value_grid(
    ui: &mut egui::Ui,
    id: impl AsIdSalt,
    data: &[(String, String)],
    alternate_stripe_order: bool,
) {
    let row_index = if alternate_stripe_order { 1 } else { 0 };
    egui::Grid::new(id)
        .with_row_color(move |row, _style| {
            if row % 2 == row_index {
                Some(Color32::from_hex("#292929").unwrap())
            } else {
                None
            }
        })
        .num_columns(2)
        .spacing([0.0, 5.0])
        .show(ui, |ui| {
            for (name, value) in data {
                ui.label(name);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.label(value);
                });
                ui.end_row();
            }
        });
}
fn collapsible_key_value_grid(
    ui: &mut egui::Ui,
    id: impl AsIdSalt,
    data: &[(String, String)],
    alternate_stripe_order: bool,
) {
    let Some((first, rest)) = data.split_first() else {
        return;
    };
    let id = ui.make_persistent_id(id);
    egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
        .show_header(ui, |ui| {
            key_value_grid(
                ui,
                "header",
                std::slice::from_ref(first),
                alternate_stripe_order,
            );
        })
        .body(|ui| {
            ui.add_space(5.0);
            key_value_grid(ui, "body", rest, !alternate_stripe_order);
        });
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
                ui.horizontal(|ui| {
                    if ui.button("Step").clicked() {
                        context.commands.push(Box::new(command::Step::new()));
                    }
                    if ui.button("Reset").clicked() {
                        context.commands.push(Box::new(command::Reset::new()));
                    }
                    if ui.button("Run").clicked() {
                        toggle(&mut context.emulator_running);
                    }
                })
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
                        self.button(ui, context, ComponentId::PpuViewWindow, "PPU View".into());
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
impl Component for PpuViewWindow {
    fn display(&mut self, ui: &mut egui::Ui, context: &mut GuiContext) -> () {
        egui::Window::new("PPU View")
            .open(
                context
                    .windows
                    .get_mut(&ComponentId::PpuViewWindow)
                    .unwrap(),
            )
            .fixed_size([200.0, 0.0])
            .resizable(false)
            .show(ui, |ui| {
                if let Some(emulator) = &context.emulator {
                    let frame = [
                        (
                            "Scanline".to_string(),
                            format!("{}", emulator.ppu_view.scanline),
                        ),
                        ("Dot".to_string(), format!("{}", emulator.ppu_view.dot)),
                    ];
                    let internal_registers = [
                        (
                            "Scroll Position:".to_string(),
                            format!("0x{:04X}", emulator.ppu_view.v),
                        ),
                        (
                            "Course X:".to_string(),
                            format!("0x{:04X}", emulator.ppu_view.t),
                        ),
                        (
                            "Fine X:".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.x),
                        ),
                        (
                            "Write Toggle:".to_string(),
                            format!("{}", emulator.ppu_view.w),
                        ),
                    ];
                    let mmio_registers = [
                        ("MMIO Registers:".to_string(), "".to_string()),
                        (
                            "PPUCTRL ($2000)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppuctrl),
                        ),
                        (
                            "PPUMASK ($2001)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppumask),
                        ),
                        (
                            "PPUSTATUS ($2002)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppustatus),
                        ),
                        (
                            "OAMADDR ($2003)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.oamaddr),
                        ),
                        (
                            "OAMDATA ($2004)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.oamdata),
                        ),
                        (
                            "PPUSCROLL ($2005)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppuscroll),
                        ),
                        (
                            "PPUADDR ($2006)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppuaddr),
                        ),
                        (
                            "PPUDATA ($2007)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppudata),
                        ),
                    ];
                    key_value_grid(ui, "internal_registers", &internal_registers, false);
                    ui.add_space(2.5);
                    collapsible_key_value_grid(ui, "mmio_registers", &mmio_registers, false);
                    ui.add_space(2.5);
                    key_value_grid(ui, "frame", &frame, true);
                }
            });
    }
}
