pub mod bus;
pub mod cartridge;
pub mod command;
pub mod console;
pub mod cpu;
pub mod ppu;

use std::cell::RefCell;
use std::rc::Rc;

pub type Shared<T> = Rc<RefCell<T>>;
