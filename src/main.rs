mod heap;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("You need to pass a path to a ROM file to load");
    }
    let rom_path = &args[1];

    let mut heap: [u8; 4096] = [0x0; 4096];
    let mut pc: u16 = heap::ROM_START;
    let _i: u16 = 0;
    let _stack: [u16; 64];
    let _frame_buffer: [[bool; 64]; 32];
    let _delay: u8;
    let _sound: u8;
    let _registers: [u8; 16] = [0; 16];

    heap::load_font(&mut heap);
    heap::load_rom(&mut heap, rom_path);
    loop {
        let op = heap::fetch_op(&heap, pc as usize);
        pc += 2;
        match op {
            0x00E0 => println!("clear screen"),
            _ => match op & 0xF000 {
                0x1000 => {
                    let jmp = op & 0x0FFF;
                    println!("jmp to {:#06X}", jmp);
                }
                0x6000 => {
                    let register = op & 0x0F00;
                    let value = op & 0x00FF;
                    println!("set V{} to {:#06X}", register, value);
                }
                0x7000 => {
                    let register = op & 0x0F00;
                    let value = op & 0x00FF;
                    println!("add V{:#06X} to {}", value, register);
                }
                0xA000 => {
                    let value = op & 0x0FFF;
                    println!("set I to {:#06X}", value);
                }
                0xD000 => {
                    let x = op & 0x0F00;
                    let y = op & 0x00F0;
                    let value = op & 0x000F;

                    println!("draw {:#06X} at X{}, Y{}", value, x, y);
                }
                _ => {
                    println!("unknown op {:#06X}", op);
                    break;
                }
            },
        }
    }
    println!("End of program");
}
