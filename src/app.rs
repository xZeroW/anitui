use crate::anime_list::{AnimeList, Status};
use crate::input::InputMode;
use color_eyre::Result;
use ratatui::style::{Color, Style};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};

pub struct App {
    pub should_exit: bool,
    pub anime_list: AnimeList,
    pub input: String,
    pub mode: InputMode,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            input: String::new(),
            anime_list: AnimeList::default(),
            mode: InputMode::Normal, // Start in normal mode
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.mode {
            InputMode::Normal => self.handle_normal_mode(key),
            InputMode::Insert => self.handle_insert_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('i') => self.mode = InputMode::Insert, // Enter Insert Mode
            KeyCode::Down => self.anime_list.select_next(),
            KeyCode::Up => self.anime_list.select_previous(),
            _ => {}
        }
    }

    fn handle_insert_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Esc => self.mode = InputMode::Normal, // Return to Normal Mode
            _ => {}
        }
    }

    /// Custom method to render the input field (not part of the Widget trait)
    fn render_input(&self, area: Rect, buf: &mut Buffer) {
        let mode_indicator = match self.mode {
            InputMode::Normal => "NORMAL",
            InputMode::Insert => "INSERT",
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!("Search [{}]", mode_indicator));

        let paragraph = Paragraph::new(self.input.as_str())
            .block(block)
            .style(Style::default().fg(Color::White));

        paragraph.render(area, buf);
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        // Example header text for the app, this can be customized
        let paragraph = Paragraph::new("AniTUI")
            .style(Style::default().fg(Color::White))
            .alignment(ratatui::layout::Alignment::Center);

        paragraph.render(area, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split the area vertically into header and main content
        let [header_area, main_area] = Layout::vertical([
            Constraint::Length(3), // Header height
            Constraint::Min(1),    // Main content height
        ])
        .areas(area);

        // Split the main area horizontally into list and details
        let [list_area, details_area] = Layout::horizontal([
            Constraint::Percentage(70), // 70% of the width for the list
            Constraint::Percentage(30), // 30% of the width for the details
        ])
        .areas(main_area);

        // Render the header
        App::render_header(header_area, buf);

        // Render the input field below the header
        let input_area = Rect::new(
            header_area.left(),
            header_area.bottom(),
            header_area.width,
            main_area.height,
        );
        self.render_input(input_area, buf);

        // Render the list on the left side
        self.anime_list.render_list(list_area, buf);

        // Render the details area on the right side
        self.anime_list.render_selected_item(details_area, buf);
    }
}
