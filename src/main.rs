mod emulator;
mod heap;
mod op_code;

use emulator::System;
use op_code::OpCode;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Missing ROM path argument example: ./chip8 roms/ibm.ch8");
    }
    let rom_path = &args[1];

    let mut system: System = System::init(rom_path);
    let mut cycle_start = Instant::now();

    loop {
        // run at 60hz
        if cycle_start.elapsed().as_micros() >= 16_600 {
            cycle_start = Instant::now();

            let op = system.fetch();

            let op_code: OpCode = op_code::decode(op);
            // println!("{}", op_code);
            if op_code == OpCode::Unknown {
                println!("Invalid OpCode {:#06X}", op);
                break;
            }

            system.execute(&op_code);
        }
    }
}
