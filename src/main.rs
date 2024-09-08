use ratatui::{backend::TestBackend, Frame, Terminal};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, StatefulImage};

struct App {
    // We need to hold the render state.
    image: Box<dyn StatefulProtocol>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend)?;

    // Should use Picker::from_termios(), to get the font size,
    // but we can't put that here because that would break doctests!
    let mut picker = Picker::new((8, 12));
    // Guess the protocol.
    picker.guess_protocol();

    // Load an image with the image crate.
    let dyn_img = image::io::Reader::open("./src/assets/111305.jpg")?.decode()?;

    // Create the Protocol which will be used by the widget.
    let image = picker.new_resize_protocol(dyn_img);

    let mut app = App { image };

    // This would be your typical `loop {` in a real app:
    terminal.draw(|f| ui(f, &mut app))?;

    Ok(())
}

fn ui(f: &mut Frame<'_>, app: &mut App) {
    // The image widget.
    let image = StatefulImage::new(None);
    // Render with the protocol state.
    f.render_stateful_widget(image, f.size(), &mut app.image);
}
