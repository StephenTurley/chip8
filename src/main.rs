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
        let op = system.fetch();

        let op_code: OpCode = op_code::decode(op);

        if event::poll(std::time::Duration::from_micros(500))? {
            if let Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        Char('q') => break,
                        Char('0') => system.set_key(Some(0x0)),
                        Char('1') => system.set_key(Some(0x1)),
                        Char('2') => system.set_key(Some(0x2)),
                        Char('3') => system.set_key(Some(0x3)),
                        Char('4') => system.set_key(Some(0x4)),
                        Char('5') => system.set_key(Some(0x5)),
                        Char('6') => system.set_key(Some(0x6)),
                        Char('7') => system.set_key(Some(0x7)),
                        Char('8') => system.set_key(Some(0x8)),
                        Char('9') => system.set_key(Some(0x9)),
                        Char('a') => system.set_key(Some(0xA)),
                        Char('b') => system.set_key(Some(0xB)),
                        Char('c') => system.set_key(Some(0xC)),
                        Char('d') => system.set_key(Some(0xD)),
                        Char('e') => system.set_key(Some(0xE)),
                        Char('f') => system.set_key(Some(0xF)),
                        _ => system.set_key(None),
                    }
                } else {
                    system.set_key(None);
                }
            }
        }
        match op_code {
            // only draw when there is a draw call
            OpCode::Cls | OpCode::Drw { vx: _, vy: _, n: _ } => {
                system.execute(&op_code);
                display.render(&system.frame_buffer);
            }
            OpCode::Unknown => {
                Display::destroy()?;
                println!("Invalid OpCode {:#06X}", op);
                break;
            }

            _ => system.execute(&op_code),
        }
    }

    Display::destroy()?;
    Ok(())
}
