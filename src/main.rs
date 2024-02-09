mod display;
mod emulator;
mod heap;
mod op_code;

use anyhow::Result;
use crossterm::event::{self, Event::Key, KeyCode::Char};
use display::Display;
use emulator::System;
use op_code::OpCode;
use std::env;

fn main() -> Result<()> {
    let mut display = Display::init()?;

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Missing ROM path argument example: ./chip8 roms/ibm.ch8");
    }
    let rom_path = &args[1];

    let mut system: System = System::init(rom_path);

    loop {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        Char('q') => break,
                        Char('0') => system.set_key(Some('0')),
                        Char('1') => system.set_key(Some('1')),
                        Char('2') => system.set_key(Some('2')),
                        Char('3') => system.set_key(Some('3')),
                        Char('4') => system.set_key(Some('4')),
                        Char('5') => system.set_key(Some('5')),
                        Char('6') => system.set_key(Some('6')),
                        Char('7') => system.set_key(Some('7')),
                        Char('8') => system.set_key(Some('8')),
                        Char('9') => system.set_key(Some('9')),
                        Char('a') => system.set_key(Some('a')),
                        Char('b') => system.set_key(Some('b')),
                        Char('c') => system.set_key(Some('c')),
                        Char('d') => system.set_key(Some('d')),
                        Char('e') => system.set_key(Some('e')),
                        Char('f') => system.set_key(Some('f')),
                        _ => system.set_key(None),
                    }
                } else {
                    system.set_key(None);
                }
            }
        }
        // run at 60hz

        let op = system.fetch();

        let op_code: OpCode = op_code::decode(op);
        // println!("{}", op_code);
        if op_code == OpCode::Unknown {
            println!("Invalid OpCode {:#06X}", op);
            break;
        }

        system.execute(&op_code);
        display.render(&system.frame_buffer);
    }

    Display::destroy()?;
    Ok(())
}
