use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};

#[derive(Debug)]
struct Anime {
    title: String,
    desc: String,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Watching,
    Completed,
    OnHold,
}

struct App {
    should_exit: bool,
    anime_list: AnimeList,
    input: String,
    mode: InputMode,
}

struct AnimeList {
    items: Vec<Anime>,
    state: ListState,
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Insert,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            input: String::new(),
            anime_list: AnimeList::from_iter([
                (Status::Watching, "Naruto", "A story about a ninja."),
                (
                    Status::Completed,
                    "Attack on Titan",
                    "A battle against titans.",
                ),
                (
                    Status::OnHold,
                    "One Piece",
                    "The adventures of a pirate crew.",
                ),
            ]),
            mode: InputMode::Normal, // Start in normal mode
        }
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for AnimeList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, title, desc)| Anime {
                status,
                title: title.to_string(),
                desc: desc.to_string(),
            })
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
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
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
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

    fn select_next(&mut self) {
        self.anime_list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.anime_list.state.select_previous();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [input_area, list_area, detail_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(5),
        ])
        .areas(area);

        self.render_input(input_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(detail_area, buf);
    }
}

impl App {
    fn render_input(&self, area: Rect, buf: &mut Buffer) {
        let mode_indicator = match self.mode {
            InputMode::Normal => "NORMAL",
            InputMode::Insert => "INSERT",
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!("Search [{}]", mode_indicator));

        let paragraph = Paragraph::new(self.input.as_str()).block(block);

        paragraph.render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Anime List");

        let items: Vec<ListItem> = self
            .anime_list
            .items
            .iter()
            .map(|anime| ListItem::new(anime.title.clone()))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">");

        StatefulWidget::render(list, area, buf, &mut self.anime_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Anime Details");

        let details = if let Some(i) = self.anime_list.state.selected() {
            let anime = &self.anime_list.items[i];
            format!(
                "Title: {}\nDescription: {}\nStatus: {:?}",
                anime.title, anime.desc, anime.status
            )
        } else {
            "No anime selected.".to_string()
        };

        let paragraph = Paragraph::new(details).block(block);

        paragraph.render(area, buf);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
