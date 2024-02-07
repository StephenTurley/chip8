mod emulator;
mod heap;
mod op_code;

use emulator::System;
use gpui::*;
use op_code::OpCode;
use std::env;
use std::time::Instant;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello {}", &self.text))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        });
    });

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
