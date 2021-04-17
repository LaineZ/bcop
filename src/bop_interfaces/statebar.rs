use console_engine::{crossterm::terminal::size, pixel, screen::Screen, Color};
use Into;

fn aabr(
    x1: u32,
    y1: u32,
    w1: u32,
    h1: u32,
    x2: u32,
    y2: u32,
    w2: u32,
    h2: u32,
) -> bool {
    return x1 < x2 + w2 && x2 < x1 + w1 && y1 < y2 + h2 && y2 < y1 + h1;
}


/// Bars widgets interface
#[derive(Clone)]
pub struct StateBarWidget {
    pub text: String,
    clicked: bool,
    x: u32,
    y: u32,
}

impl StateBarWidget {
    pub fn new() -> Self {
        StateBarWidget {
            text: String::new(),
            clicked: false,
            x: 0,
            y: 0,
        }
    }

    fn update_click(&mut self, x: u32, y: u32) {
        self.clicked = aabr(x, y, self.text.len() as u32, 1, self.x, self.y, 1, 1);
    }


    pub fn set_text<T: AsRef<str>>(&mut self, item: T) {
        self.text = item.as_ref().to_string();
    }
}


/// Bars interface
#[derive(Clone)]
pub struct StateBar {
    /// Widgets
    pub widgets: Vec<StateBarWidget>,
    /// Is error occured? Set header bar background color to red
    error: bool,
    /// Screen struct
    screen: Screen,
    /// Y position of the bar
    pub y: u32,
}

impl StateBar {
    /// Creates a state bars
    pub fn new() -> Self {
        let (cols, rows) =
            size().expect("Unable to get terminal size continue work is not available!");

        Self {
            widgets: Vec::new(),
            error: false,
            screen: Screen::new(cols.into(), 1),
            y: rows as u32,
        }
    }

    /// Draws a bar and its widgets
    pub fn draw(&mut self) -> &Screen {
        self.screen.fill(pixel::pxl_bg(' ', Color::Blue));
        let mut xpos = 0;
        for wid in self.widgets.iter() {
            log::info!("{}x{} - {}", xpos, self.y, wid.text);
            self.screen.print_fbg(xpos, 0, format!("{}|", wid.text).as_str(), Color::White, Color::Blue);
            xpos += wid.text.chars().collect::<String>().len() as i32;
        }
        &self.screen
    }

    /// Updates widgets in bar
    pub fn update_click(&mut self, x: u32, y: u32) {
        for wid in self.widgets.iter_mut() {
            wid.update_click(x, y);
        }
    }

    /// Resizes bar
    pub fn resize(&mut self, w: u16, h: u16) {
        self.screen.clear();
        self.screen.resize(w as u32, 1);
        self.y = h as u32;
    }
}
