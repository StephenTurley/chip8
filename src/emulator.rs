use crate::heap;
use crate::heap::Heap;

pub struct System {
    heap: Heap,
    pc: u16,
    // i: u16,
    // stack: [u16; 64],
    // frame_buffer: [[bool; 64]; 32],
    // delay: u8,
    // sound: u8,
    // registers: [u8; 16],
}
impl System {
    pub fn init(rom_path: &String) -> System {
        let mut system = System {
            heap: Heap::new(),
            pc: heap::ROM_START,
            // i: 0,
            // stack: [0; 64],
            // frame_buffer: [[false; 64]; 32],
            // delay: 0,
            // sound: 0,
            // registers: [0; 16],
        };
        system.heap.load_font();
        system.heap.load_rom(rom_path);
        system
    }

    pub fn fetch(&mut self) -> u16 {
        let op = self.heap.fetch_op(self.pc as usize);
        self.pc += 2;
        op
    }
}
