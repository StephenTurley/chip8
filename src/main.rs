mod emulator;
mod heap;
mod op_code;

use emulator::System;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("You need to pass a path to a ROM file to load");
    }
    let rom_path = &args[1];

    // let mut heap: [u8; 4096] = [0x0; 4096];
    // let mut pc: u16 = heap::ROM_START;
    // let _i: u16 = 0;
    // let _stack: [u16; 64];
    // let _frame_buffer: [[bool; 64]; 32];
    // let _delay: u8;
    // let _sound: u8;
    // let _registers: [u8; 16] = [0; 16];
    //
    let mut system: System = System::init(rom_path);

    loop {
        // fetch
        let op = system.fetch();

        // decode
        let op_code: op_code::OpCode = op_code::decode(op);
        if op_code == op_code::OpCode::Unknown {
            break;
        }

        //execute
        dbg!(op_code);
    }
    println!("End of program");
}
