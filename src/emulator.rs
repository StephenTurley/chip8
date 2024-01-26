use crate::heap;
use crate::heap::Heap;
use crate::OpCode;

pub struct System {
    heap: Heap,
    pc: u16,
    i: u16,
    // stack: [u16; 64],
    frame_buffer: [[bool; 64]; 32],
    // delay: u8,
    // sound: u8,
    registers: [u8; 16],
}
impl System {
    pub fn new() -> System {
        System {
            heap: Heap::new(),
            pc: heap::ROM_START,
            i: 0,
            // stack: [0; 64],
            frame_buffer: [[false; 64]; 32],
            // delay: 0,
            // sound: 0,
            registers: [0; 16],
        }
    }

    pub fn init(rom_path: &String) -> System {
        let mut system = Self::new();
        system.heap.load_font();
        system.heap.load_rom(rom_path);
        system
    }

    pub fn fetch(&mut self) -> u16 {
        let op = self.heap.fetch_op(self.pc as usize);
        self.pc += 2;
        op
    }

    pub fn execute(&mut self, op: &OpCode) {
        match *op {
            OpCode::CLS => {
                self.frame_buffer = [[false; 64]; 32];
                println!("CLS called");
            }
            OpCode::JMP(addr) => {
                self.pc = addr;
                println!("JMP called with address {:#06X}", addr);
            }
            OpCode::LDVx { vx, value } => {
                self.registers[vx as usize] = value;
                println!(
                    "LDV called for register V{:#06X}, and value {:#06X}",
                    vx, value
                );
            }
            OpCode::ADDVx { vx, value } => {
                self.registers[vx as usize] += value;
                println!(
                    "ADD called for register V{:#06X} and value {:#06X}",
                    vx, value
                );
            }
            OpCode::LDI(value) => {
                self.i = value;
                println!("LDI called with value {:#06X}", value);
            }
            OpCode::DRW { vx, vy, value } => {
                println!(
                    "DRW called with VX{:#06X}, VY{:#06X}, and value  {:#06X}",
                    vx, vy, value
                );
            }
            OpCode::Unknown => {
                println!("Unknown OpCode");
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fetch() {
        let mut system = System::new();
        // CLS
        system.heap.set_byte(0x0200, 0x0000);
        system.heap.set_byte(0x0201, 0x00E0);

        let result = system.fetch();

        assert_eq!(result, 0x00E0);
        assert_eq!(system.pc, 0x0202);
    }

    #[test]
    fn cls() {
        let mut system = System {
            frame_buffer: [[true; 64]; 32],
            ..System::new()
        };

        system.execute(&OpCode::CLS);

        assert_eq!([[false; 64]; 32], system.frame_buffer);
    }

    #[test]
    fn jmp() {
        let mut system = System::new();

        system.execute(&OpCode::JMP(0x0555));

        assert_eq!(0x0555, system.pc);
    }

    #[test]
    fn ld_vx() {
        let mut system = System::new();

        system.execute(&OpCode::LDVx {
            vx: 0x000F,
            value: 0x0012,
        });

        assert_eq!(0x0012, system.registers[0x000F]);
    }

    #[test]
    fn add_vx() {
        let mut system = System::new();
        system.registers[0x000A] = 0x00AB;

        system.execute(&OpCode::ADDVx {
            vx: 0x000A,
            value: 0x0001,
        });

        assert_eq!(0x00AC, system.registers[0x000A]);
    }

    #[test]
    fn ldi() {
        let mut system = System::new();

        system.execute(&OpCode::LDI(0x0123));

        assert_eq!(0x0123, system.i);
    }
}
