use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Stdout};

pub struct Display {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Display {
    pub fn init() -> Result<Display> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Display { terminal })
    }

    pub fn destroy() -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn render(&mut self, _frame_buffer: &[[bool; 64]; 32]) {
        self.terminal
            .draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("Hello world (press q to quit)")
                        .white()
                        .on_blue(),
                    area,
                );
            })
            .unwrap();

        // print!("\x1B[2J"); // clear terminal
        // for row in *frame_buffer {
        //     println!();
        //     for px in row {
        //         let symbol = if px { "⚫" } else { "⚪" };
        //
        //         print!("{}", symbol);
        //     }
        // }
        // println!();
    }
}
