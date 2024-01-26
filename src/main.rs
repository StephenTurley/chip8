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
        panic!("You need to pass a path to a ROM file to load");
    }
    let rom_path = &args[1];

    let mut system: System = System::init(rom_path);
    let mut cycle_start = Instant::now();

    loop {
        // run at 60hz
        if cycle_start.elapsed().as_micros() >= 16_600 {
            cycle_start = Instant::now();
            // fetch
            let op = system.fetch();

            // decode
            let op_code: OpCode = op_code::decode(op);
            if op_code == OpCode::Unknown {
                break;
            }

            //execute
            system.execute(&op_code);
        }
    }
    println!("End of program");
}
