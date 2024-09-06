use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Ongoing,
    Completed,
}

pub struct AnimeList {
    pub items: Vec<AnimeItem>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct AnimeItem {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Default for AnimeList {
    fn default() -> Self {
        Self {
            items: vec![
                AnimeItem {
                    name: "Attack on Titan".to_string(),
                    description: "Humans fighting titans to survive".to_string(),
                    status: Status::Ongoing,
                },
                AnimeItem {
                    name: "One Piece".to_string(),
                    description: "Pirate adventures to find the ultimate treasure".to_string(),
                    status: Status::Completed,
                },
                AnimeItem {
                    name: "Naruto".to_string(),
                    description: "Ninja striving to become Hokage".to_string(),
                    status: Status::Ongoing,
                },
            ],
            state: ListState::default(),
        }
    }
}

impl AnimeList {
    pub fn select_next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select_previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Anime List")
            .borders(ratatui::widgets::Borders::ALL);

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| ListItem::new(format!("{} - {:?}", item.name, item.status)))
            .collect();

        let list = List::new(items).block(block).highlight_symbol(">>");

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    pub fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let selected = self
            .state
            .selected()
            .map(|i| {
                format!(
                    "Selected: {}\nDescription: {}\nStatus: {:?}",
                    self.items[i].name, self.items[i].description, self.items[i].status
                )
            })
            .unwrap_or_else(|| "No item selected".to_string());

        let paragraph = ratatui::widgets::Paragraph::new(format!("{}", selected)).block(
            Block::default()
                .title("Details")
                .borders(ratatui::widgets::Borders::ALL),
        );

        paragraph.render(area, buf);
    }
}
