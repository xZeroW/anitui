use crate::anime_list::{AnimeList, Status};
use crate::input::InputMode;
use color_eyre::Result;
use ratatui::style::{Color, Style};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
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
            KeyCode::Down | KeyCode::Up => {
                self.mode = InputMode::Normal;
                self.anime_list.select_next();
            }
            KeyCode::Enter => {
                self.add_anime(self.input.clone(), String::new(), Status::Ongoing);
                self.input.clear();
                self.mode = InputMode::Normal;
            }
            _ => {}
        }
    }

    /// Custom method to render the input field (not part of the Widget trait)
    fn render_input(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Search");

        let paragraph = Paragraph::new(self.input.as_str())
            .block(block)
            .style(match self.mode {
                InputMode::Normal => Style::default(),
                InputMode::Insert => Style::default().fg(Color::LightBlue),
            });

        paragraph.render(area, buf);
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        // Example header text for the app, this can be customized
        let paragraph = Paragraph::new("AniTUI")
            .style(Style::default())
            .alignment(ratatui::layout::Alignment::Center);

        paragraph.render(area, buf);
    }

    fn add_anime(&mut self, name: String, description: String, status: Status) {
        self.anime_list.add_item(name, description, status);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split the area vertically into header and main content
        let [header_area, input_area, main_area] = Layout::vertical([
            Constraint::Length(1), // Header height
            Constraint::Length(3), // Input field height
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

        self.render_input(input_area, buf);

        // Render the list on the left side
        self.anime_list.render_list(list_area, buf, &self.mode);

        // Render the details area on the right side
        self.anime_list.render_selected_item(details_area, buf);
    }
}
