use crate::heap;
use crate::heap::Heap;
use crate::OpCode;

pub struct System {
    heap: Heap,
    pc: u16,
    i: u16,
    // stack: [u16; 64],
    frame_buffer: [[bool; 64]; 32], //indexed [y][x]; top left [0][0]; bottom right [31, 63]
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
                println!("CLS called");
                self.frame_buffer = [[false; 64]; 32];
            }
            OpCode::JMP(addr) => {
                self.pc = addr;
                // println!("JMP called with address {:#06X}", addr);
            }
            OpCode::LDVx { vx, value } => {
                println!(
                    "LDV called for register V{:#06X}, and value {:#06X}",
                    vx, value
                );
                self.registers[vx as usize] = value;
            }
            OpCode::ADDVx { vx, value } => {
                println!(
                    "ADD called for register V{:#06X} and value {:#06X}",
                    vx, value
                );
                self.registers[vx as usize] += value;
            }
            OpCode::LDI(value) => {
                println!("LDI called with value {:#06X}", value);
                self.i = value;
            }
            OpCode::DRW { vx, vy, n } => {
                println!(
                    "DRW called with VX{:#06X}, VY{:#06X}, and value  {:#06X}",
                    vx, vy, n
                );
                self.update_frame_buffer(vx as usize, vy as usize, n as usize);
                self.render();
            }
            OpCode::Unknown => {
                println!("Unknown OpCode");
            }
        };
    }

    fn render(&self) {
        for row in self.frame_buffer {
            print!("\n");
            for px in row {
                let symbol = if px { "⚫" } else { "⚪" };

                print!("{}", symbol);
            }
        }
        print!("\n");
    }

    fn update_frame_buffer(&mut self, vx: usize, vy: usize, sprite_rows: usize) {
        //fetch screen coordinates
        let start_x = self.registers[vx] % 64; // allow the start_x to wrap using modulo
        let start_y = self.registers[vy] % 32; // allow the start_y to wrap using modulo

        let sprite_ref: usize = self.i.into();

        //set collision to 0
        self.registers[0x000F as usize] = 0;

        for row in 0..sprite_rows {
            let y = start_y + row as u8;
            let sprite_row: u8 = self.heap.fetch_byte(sprite_ref + row);
            for bit_index in 0..8 {
                let x = start_x + bit_index;
                // to get the current pixel we want to convert the bit at bit_index to a bool
                // shift bits in the row to the left until the current bit is at the least significant position
                // mask all other bits out
                // convert to bool by != 0
                if x < 64 && y < 32 {
                    let pixel = ((sprite_row >> 7 - bit_index) & 0x01) != 0;
                    let old_pixel = self.frame_buffer[y as usize][x as usize];
                    let new_pixel = old_pixel ^ pixel;

                    // if the current pixel collides with old_pixel, set the collision flag
                    if old_pixel && pixel {
                        self.registers[0xF] = 1;
                    }

                    self.frame_buffer[y as usize][x as usize] = new_pixel;
                }
            }
        }
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
