pub mod cpu;
pub mod ppu;
pub mod bus;
pub mod cartridge;
pub mod console;
pub mod debug;

use std::cell::RefCell;
use std::rc::Rc;

pub type Shared<T> = Rc<RefCell<T>>;
