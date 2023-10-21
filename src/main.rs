use std::io::{stderr, Result};

use crossterm::ExecutableCommand;
use crossterm::event::{self, KeyCode, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Paragraph;

fn main() -> Result<()> {

    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;


    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Welcome to Git Branch Cleaner! (Leave by pressing q)"),
                area
            )
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(event::KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char(char),
                ..
            }) = event::read()? {
                match char {
                    'q' => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

