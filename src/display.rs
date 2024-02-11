use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::Flex,
    prelude::*,
    style::Color,
    symbols,
    widgets::{
        block,
        canvas::{Canvas, Context, Rectangle},
        Block, Borders,
    },
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

    pub fn render(&mut self, frame_buffer: &[[bool; 64]; 32]) {
        self.terminal
            .draw(|frame| {
                let area = centered_rect(frame.size(), 128, 32);
                frame.render_widget(
                    Canvas::default()
                        .marker(symbols::Marker::HalfBlock)
                        .block(
                            Block::default()
                                .title(block::Title::from("CHIP-8").alignment(Alignment::Center))
                                .borders(Borders::ALL),
                        )
                        .x_bounds([0.0, 64.0])
                        .y_bounds([0.0, 32.0])
                        .paint(|ctx| {
                            render_frame_buffer(frame_buffer, ctx);
                        }),
                    area,
                )
            })
            .unwrap();
    }
}

fn render_frame_buffer(frame_buffer: &[[bool; 64]; 32], ctx: &mut Context<'_>) {
    for (y, row) in frame_buffer.iter().enumerate() {
        for (x, px) in row.iter().enumerate() {
            if *px {
                draw_pixel(ctx, x as f64, (31 - y) as f64);
            }
        }
    }
}

fn draw_pixel(ctx: &mut Context, x: f64, y: f64) {
    ctx.draw(&Rectangle {
        x,
        y,
        width: 1.0,
        height: 1.0,
        color: Color::LightGreen,
    });
}
fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let horizontal = Layout::horizontal([width]).flex(Flex::Center);
    let vertical = Layout::vertical([height]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
