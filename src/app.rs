use crate::anime_list::{AnimeList, Status};
use crate::input::InputMode;
use color_eyre::Result;
use image::DynamicImage;
use ratatui::style::{Color, Style};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;

pub struct App {
    pub should_exit: bool,
    pub anime_list: AnimeList,
    pub input: String,
    pub mode: InputMode,
    pub image: Box<dyn StatefulProtocol>,
}

impl Default for App {
    fn default() -> Self {
        let mut picker = Picker::new((8, 12));
        picker.guess_protocol();

        // Create a placeholder image for initialization
        let placeholder_image = DynamicImage::new_rgba8(1, 1);
        let image_protocol = picker.new_resize_protocol(placeholder_image);

        Self {
            should_exit: false,
            input: String::new(),
            anime_list: AnimeList::default(),
            mode: InputMode::Normal,
            image: image_protocol,
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

            // After every key press, update the image according to the selected anime
            self.update_image();
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

    pub fn update_image(&mut self) {
        let mut picker = Picker::new((8, 12)); // Adjust the font size
        picker.guess_protocol();

        if let Some(selected_image) = self.anime_list.selected_image() {
            // Create a new protocol for the selected anime's image
            self.image = picker.new_resize_protocol(selected_image.clone());
        }
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
        let [list_area, details_area, image_area] = Layout::horizontal([
            Constraint::Percentage(50), // 50% for the list
            Constraint::Percentage(30), // 30% for the details
            Constraint::Percentage(20), // 20% for the image
        ])
        .areas(main_area);

        // Render the input field below the header
        App::render_header(header_area, buf);
        self.render_input(header_area, buf); // Render the input field

        // Render the list on the left side
        self.anime_list.render_list(list_area, buf);

        // Render the details area on the right side
        self.anime_list.render_selected_item(details_area, buf);

        self.anime_list.render_image(image_area, buf)
    }
}
