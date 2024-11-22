use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui;
use ratatui::text::{Line, Text};
use ratatui::DefaultTerminal;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    widgets::{Block, Paragraph, Widget},
    Frame,
};
use std::io;

pub fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Press any key to leave this shit");
    frame.render_widget(text, frame.area());
}
