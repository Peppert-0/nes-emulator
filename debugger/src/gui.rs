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
pub struct ControlsWindow {
    #[builder(default = 1)]
    step_times: u32,
}
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
                    let flags = Vec::from([
                        RowType::Value((
                            "Flags:".to_string(),
                            format!("0x{:02X}", emulator.cpu_view.registers.p),
                        )),
                        RowType::Value((
                            "Carry:".to_string(),
                            format!("{}", emulator.cpu_view.flags.carry),
                        )),
                        RowType::Value((
                            "Zero:".to_string(),
                            format!("{}", emulator.cpu_view.flags.carry),
                        )),
                        RowType::Value((
                            "Interrupt\nDisable:".to_string(),
                            format!("{}", emulator.cpu_view.flags.interrupt_disable),
                        )),
                        RowType::Value((
                            "Decimal:".to_string(),
                            format!("{}", emulator.cpu_view.flags.decimal),
                        )),
                        RowType::Value((
                            "Overflow:".to_string(),
                            format!("{}", emulator.cpu_view.flags.overflow),
                        )),
                        RowType::Value((
                            "Negative:".to_string(),
                            format!("{}", emulator.cpu_view.flags.negative),
                        )),
                    ]);

                    let registers = [
                        RowType::Value((
                            "Program Counter:".to_string(),
                            format!("0x{:04X}", emulator.cpu_view.registers.pc),
                        )),
                        RowType::Value((
                            "Stack Pointer:".to_string(),
                            format!("0x{:02X}", emulator.cpu_view.registers.sp),
                        )),
                        RowType::Value((
                            "Accumulator:".to_string(),
                            format!("0x{:02X}", emulator.cpu_view.registers.a),
                        )),
                        RowType::Value((
                            "X Register:".to_string(),
                            format!("0x{:02X}", emulator.cpu_view.registers.x),
                        )),
                        RowType::Value((
                            "Y Register:".to_string(),
                            format!("0x{:02X}", emulator.cpu_view.registers.y),
                        )),
                        RowType::Collapsible(flags),
                    ];

                    key_value_grid(ui, &registers, false);
                }
            });
    }
}
enum RowType {
    Value((String, String)),
    Collapsible(Vec<RowType>),
}
fn key_value_grid(ui: &mut egui::Ui, data: &[RowType], alternate_stripe_order: bool) {
    let row_index = if alternate_stripe_order { 1 } else { 0 };
    let sections = data.chunk_by(|a, b| std::mem::discriminant(a) == std::mem::discriminant(b));
    for section in sections {
        match &section[0] {
            RowType::Value((name, value)) => {
                egui::Grid::new(name)
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
                        for row in section {
                            match row {
                                RowType::Value((name, value)) => {
                                    ui.label(name);
                                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                        ui.set_min_width(ui.available_width());
                                        ui.label(value);
                                    });
                                    ui.end_row();
                                }
                                RowType::Collapsible(sub_data) => {}
                            }
                        }
                    });
            }
            RowType::Collapsible(sub_data) => {
                collapsible_key_value_grid(ui, sub_data, false);
            }
        };
    }
}
fn collapsible_key_value_grid(ui: &mut egui::Ui, data: &[RowType], alternate_stripe_order: bool) {
    let Some((first, rest)) = data.split_first() else {
        return;
    };
    match first {
        RowType::Value((name, value)) => {
            let id = egui::Id::new(name);
            egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
                .show_header(ui, |ui| {
                    key_value_grid(ui, std::slice::from_ref(first), alternate_stripe_order);
                })
                .body(|ui| {
                    ui.add_space(5.0);
                    key_value_grid(ui, rest, !alternate_stripe_order);
                });
        }
        RowType::Collapsible((sub_data)) => {}
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
                ui.horizontal(|ui| {
                    if ui.button("Step").clicked() {
                        context
                            .commands
                            .push(Box::new(command::Step::new().times(self.step_times)));
                    }
                    ui.add(
                        egui::Slider::new(&mut self.step_times, 1..=5_000_000).logarithmic(true),
                    );
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
                if let Some(emulator) = &context.emulator
                    && !emulator.cpu_view.instructions.is_empty()
                {
                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .show_rows(
                            ui,
                            ui.text_style_height(&egui::TextStyle::Body),
                            emulator.cpu_view.instructions.len() - 1,
                            |ui, row_range| {
                                egui::Grid::new("cpu_trace")
                                    .striped(true)
                                    .num_columns(3)
                                    .show(ui, |ui| {
                                        for instruction in
                                            emulator.cpu_view.instructions.range(row_range)
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
                            },
                        );
                }
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
            .default_size([200.0, 0.0])
            .resizable([true, false])
            .show(ui, |ui| {
                if let Some(emulator) = &context.emulator {
                    let ppuctrl = Vec::from([
                        RowType::Value((
                            "PPUCTRL ($2000)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppuctrl.value),
                        )),
                        RowType::Value((
                            "Base nametable address".to_string(),
                            emulator
                                .ppu_view
                                .mmio
                                .ppuctrl
                                .base_nametable_address
                                .clone(),
                        )),
                        RowType::Value((
                            "VRAM address increment".to_string(),
                            emulator
                                .ppu_view
                                .mmio
                                .ppuctrl
                                .vram_address_increment
                                .clone(),
                        )),
                        RowType::Value((
                            "Sprite pattern\ntable address".to_string(),
                            emulator
                                .ppu_view
                                .mmio
                                .ppuctrl
                                .sprite_pattern_table_address
                                .clone(),
                        )),
                        RowType::Value((
                            "Background pattern\ntable address".to_string(),
                            emulator
                                .ppu_view
                                .mmio
                                .ppuctrl
                                .background_pattern_table_address
                                .clone(),
                        )),
                        RowType::Value((
                            "Sprite size".to_string(),
                            emulator.ppu_view.mmio.ppuctrl.sprite_size.clone(),
                        )),
                        RowType::Value((
                            "Vblank NMI".to_string(),
                            emulator.ppu_view.mmio.ppuctrl.vblank_nmi.clone(),
                        )),
                    ]);
                    let ppustatus = Vec::from([
                        RowType::Value((
                            "PPUSTATUS ($2002)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppustatus.value),
                        )),
                        RowType::Value((
                            "Sprite overflow".to_string(),
                            format!("{}", emulator.ppu_view.mmio.ppustatus.sprite_overflow),
                        )),
                        RowType::Value((
                            "Sprite 0 hit".to_string(),
                            format!("{}", emulator.ppu_view.mmio.ppustatus.sprite_0_hit),
                        )),
                        RowType::Value((
                            "Vblank".to_string(),
                            format!("{}", emulator.ppu_view.mmio.ppustatus.vblank),
                        )),
                    ]);
                    let mmio_registers = Vec::from([
                        RowType::Value(("MMIO Registers".to_string(), "".to_string())),
                        RowType::Collapsible(ppuctrl),
                        RowType::Value((
                            "PPUMASK ($2001)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppumask),
                        )),
                        RowType::Collapsible(ppustatus),
                        RowType::Value((
                            "OAMADDR ($2003)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.oamaddr),
                        )),
                        RowType::Value((
                            "OAMDATA ($2004)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.oamdata),
                        )),
                        RowType::Value((
                            "PPUSCROLL ($2005)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppuscroll),
                        )),
                        RowType::Value((
                            "PPUADDR ($2006)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppuaddr),
                        )),
                        RowType::Value((
                            "PPUDATA ($2007)".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.mmio.ppudata),
                        )),
                    ]);
                    let ppu_view = Vec::from([
                        RowType::Value((
                            "Scroll Position".to_string(),
                            format!("0x{:04X}", emulator.ppu_view.v),
                        )),
                        RowType::Value((
                            "Course X".to_string(),
                            format!("0x{:04X}", emulator.ppu_view.t),
                        )),
                        RowType::Value((
                            "Fine X".to_string(),
                            format!("0x{:02X}", emulator.ppu_view.x),
                        )),
                        RowType::Value((
                            "Write Toggle".to_string(),
                            format!("{}", emulator.ppu_view.w),
                        )),
                        RowType::Collapsible(mmio_registers),
                        RowType::Value((
                            "Scanline".to_string(),
                            format!("{}", emulator.ppu_view.scanline),
                        )),
                        RowType::Value(("Dot".to_string(), format!("{}", emulator.ppu_view.dot))),
                    ]);
                    key_value_grid(ui, &ppu_view, false);
                }
            });
    }
}
