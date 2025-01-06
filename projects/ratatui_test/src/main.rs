use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    prelude::*,
    widgets::*,
    DefaultTerminal,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut progress: f32 = 0.0;
    loop {
        terminal.draw(|frame| {
            let gauge = Gauge::default()
                .block(
                    Block::bordered()
                        .border_style(Style::new().red())
                        .title_top(Line::from("Progess").centered())
                        .title_bottom(Line::from("Press 'q' to exit").centered()),
                )
                .gauge_style(
                    Style::default()
                        .on_black()
                        .green()
                        .add_modifier(Modifier::RAPID_BLINK),
                )
                .percent(progress.round() as u16);

            let gauge_width = 72;
            let gauge_height = 3;
            let x = (frame.area().width).saturating_sub(gauge_width) / 2;
            let y = (frame.area().height).saturating_sub(gauge_height) / 2;

            let gauge_area = Rect::new(x, y, gauge_width, gauge_height).clamp(frame.area());

            frame.render_widget(gauge, gauge_area);
        })?;

        progress = if progress >= 100.0 {
            0.0
        } else {
            progress + 2.0
        };

        // Waiting for the Event is blocking, using event::poll solves the issue.
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
