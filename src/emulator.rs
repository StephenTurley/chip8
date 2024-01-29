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
    v: [u8; 16],
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
            v: [0; 16],
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
            OpCode::CLS => self.frame_buffer = [[false; 64]; 32],
            OpCode::JMP(addr) => self.pc = addr,
            OpCode::LDVx { vx, value } => self.v[vx] = value,
            OpCode::LDVxVy { vx, vy } => self.v[vx] = self.v[vy],
            OpCode::ORVxVy { vx, vy } => self.v[vx] = self.v[vx] | self.v[vy],
            OpCode::ANDVxVy { vx, vy } => self.v[vx] = self.v[vx] & self.v[vy],
            OpCode::XORVxVy { vx, vy } => self.v[vx] = self.v[vx] ^ self.v[vy],
            OpCode::ADDVxVy { vx, vy } => self.v[vx] = self.v[vx] + self.v[vy],
            OpCode::SUB { vx, vy } => {
                let x = self.v[vx];
                let y = self.v[vy];
                self.v[vx] = x.wrapping_sub(y);
                self.v[0x000F] = if x > y { 1 } else { 0 };
            }
            OpCode::SUBn { vx, vy } => {
                let x = self.v[vx];
                let y = self.v[vy];
                self.v[vx] = y.wrapping_sub(x);
                self.v[0x000F] = if y > x { 1 } else { 0 };
            }
            OpCode::SHR { vx, vy: _vy } => {
                // this impl ignores the vy... TODO configure this for
                // some roms that use the older version that moved vy
                // to vx then shifted
                let x = self.v[vx];
                let lsd = x & 0x01;
                self.v[0xF] = lsd;
                self.v[vx] = x >> 1;
            }
            OpCode::ADDVx { vx, value } => self.v[vx] += value,
            OpCode::LDI(value) => self.i = value,
            OpCode::DRW { vx, vy, n } => {
                self.update_frame_buffer(vx, vy, n);
                self.render();
            }
            OpCode::Unknown => {}
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
        let start_x = self.v[vx] % 64; // allow the start_x to wrap using modulo
        let start_y = self.v[vy] % 32; // allow the start_y to wrap using modulo

        let sprite_ref: usize = self.i.into();

        //set collision to 0
        self.v[0x000F as usize] = 0;

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
                        self.v[0xF] = 1;
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
    fn add_vx() {
        let mut system = System::new();
        system.v[0x000A] = 0x00AB;

        system.execute(&OpCode::ADDVx {
            vx: 0x000A,
            value: 0x0001,
        });

        assert_eq!(0x00AC, system.v[0x000A]);
    }

    #[test]
    fn ld_vx() {
        let mut system = System::new();

        system.execute(&OpCode::LDVx {
            vx: 0x000F,
            value: 0x0012,
        });

        assert_eq!(0x0012, system.v[0x000F]);
    }

    #[test]
    fn ld_vx_vy() {
        let mut system = System::new();
        system.v[0x000A] = 0xBE;

        system.execute(&OpCode::LDVxVy {
            vx: 0x000F,
            vy: 0x000A,
        });

        assert_eq!(0xBE, system.v[0x000F]);
    }

    #[test]
    fn or_vx_vy() {
        let mut system = System::new();
        system.v[0x000F] = 0xF0;
        system.v[0x000A] = 0x0F;

        system.execute(&OpCode::ORVxVy {
            vx: 0x000F,
            vy: 0x000A,
        });

        assert_eq!(0xFF, system.v[0x000F]);
    }

    #[test]
    fn and_vx_vy() {
        let mut system = System::new();
        system.v[0x000F] = 0xFF;
        system.v[0x000A] = 0x1F;

        system.execute(&OpCode::ANDVxVy {
            vx: 0x000F,
            vy: 0x000A,
        });

        assert_eq!(0x1F, system.v[0x000F]);
    }

    #[test]
    fn xor_vx_vy() {
        let mut system = System::new();
        system.v[0x000F] = 0x44; // 01000100
        system.v[0x000A] = 0x51; // 01010001 XOR
                                 // ------------
                                 // 00010101
                                 // 0x15

        system.execute(&OpCode::XORVxVy {
            vx: 0x000F,
            vy: 0x000A,
        });

        assert_eq!(0x15, system.v[0x000F]);
    }

    #[test]
    fn add_vx_vy() {
        let mut system = System::new();
        system.v[0x000F] = 0x05;
        system.v[0x000A] = 0x01;

        system.execute(&OpCode::ADDVxVy {
            vx: 0x000F,
            vy: 0x000A,
        });

        assert_eq!(0x06, system.v[0x000F]);
    }

    #[test]
    fn sub() {
        // 8xy5 - SUB Vx, Vy
        // Set Vx = Vx - Vy, set VF = NOT borrow.
        // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
        let mut system = System::new();
        system.v[0x000D] = 0x05; //vx
        system.v[0x000A] = 0x01; //vy

        system.execute(&OpCode::SUB {
            vx: 0x000D,
            vy: 0x000A,
        });

        assert_eq!(0x04, system.v[0x000D]);
        assert_eq!(0x1, system.v[0x000F], "must set borrow bit");

        let mut system = System::new();
        system.v[0x000D] = 0x01; //vx
        system.v[0x000A] = 0x05; //vy

        system.execute(&OpCode::SUB {
            vx: 0x000D,
            vy: 0x000A,
        });

        assert_eq!(0xFC, system.v[0x000D]);
        assert_eq!(0x0, system.v[0x000F], "do not set borrow bit if x > y");
    }

    #[test]
    fn subn() {
        // Set Vx = Vy - Vx, set VF = NOT borrow.
        // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
        //
        let mut system = System::new();
        system.v[0x000D] = 0x01; //vx
        system.v[0x000A] = 0x05; //vy

        system.execute(&OpCode::SUBn {
            vx: 0x000D,
            vy: 0x000A,
        });

        assert_eq!(0x04, system.v[0x000D]);
        assert_eq!(0x1, system.v[0x000F], "must set borrow bit");

        let mut system = System::new();
        system.v[0x000D] = 0x05; //vx
        system.v[0x000A] = 0x01; //vy

        system.execute(&OpCode::SUBn {
            vx: 0x000D,
            vy: 0x000A,
        });

        assert_eq!(0xFC, system.v[0x000D]);
        assert_eq!(0x0, system.v[0x000F], "do not set borrow bit if x > y");
    }

    #[test]
    fn shr() {
        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
        let mut system = System::new();
        system.v[0x000A] = 0x05; // vx 00000101
        system.v[0x000B] = 0x01; // vy, ignored in this impl

        system.execute(&OpCode::SHR {
            vx: 0x000A,
            vy: 0x000B,
        });

        assert_eq!(0x02, system.v[0x000A]);
        assert_eq!(
            0x01, system.v[0x000F],
            "should set v[0xF] to 1 since its the LSD"
        );
    }

    #[test]
    fn ldi() {
        let mut system = System::new();

        system.execute(&OpCode::LDI(0x0123));

        assert_eq!(0x0123, system.i);
    }
}
