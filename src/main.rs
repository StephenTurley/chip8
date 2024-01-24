mod heap;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("You need to pass a path to a ROM file to load");
    }
    let rom_path = &args[1];

    let mut heap: [u8; 4096] = [0x0; 4096];
    let _pc: u16;
    let _i: u16;
    let _stack: [u16; 64];
    let _frame_buffer: [[bool; 64]; 32];
    let _delay: u8;
    let _sound: u8;
    let _registers: [u8; 16];

    heap::load_font(&mut heap);
    heap::load_rom(&mut heap, rom_path);
    let rom = &heap[0x200..0x220];
    dbg!(rom);
}
