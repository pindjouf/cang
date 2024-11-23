use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::Constraint::{Fill, Length, Min};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, BorderType, Borders, List, Padding, Paragraph, Tabs, Widget},
    DefaultTerminal, Frame,
};
use std::io;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Default)]
pub struct App {
    state: AppState,
    selected_tab: SelectedTab,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "VMs")]
    Tab1,
    #[strum(to_string = "System")]
    Tab2,
    #[strum(to_string = "Network")]
    Tab3,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                KeyCode::Char('q') => self.quit(),
                _ => {}
            }
        }
        Ok(())
    }

    fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl SelectedTab {
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    fn next(self) -> Self {
        let current_index: usize = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = frame.area();
        let titles = vec!["VMs", "System", "Network"];
        let items = vec!["Create VM", "List VMs", "Edit VM"];

        let vertical = Layout::vertical([Length(3), Min(0), Length(1)]);
        let [nav_area, main_area, status_area] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Fill(1); 2]);
        let [left_area, right_area] = horizontal.areas(main_area);

        let block = Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .title("Navigation bar");

        let tabs = Tabs::new(titles)
            .block(block)
            .divider("|")
            .select(app_state.selected_tab);
        frame.render_widget(tabs, nav_area);

        let content_title = match app_state.selected_tab {
            Tab1 => "Virtual Machines",
            Tab2 => "System Hardening",
            Tab3 => "Network Management",
            _ => unreachable!(),
        };

        let preview_title = match app_state.selected_tab {
            Tab1 => items[0],
            Tab2 => items[1],
            Tab3 => items[2],
            _ => unreachable!(),
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .title(content_title)
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL),
            )
            .highlight_symbol(">>");
        frame.render_widget(list, left_area);

        let preview = Paragraph::new("yoo").block(
            Block::default()
                .title(preview_title)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        );
        frame.render_widget(preview, right_area);
    }
}
