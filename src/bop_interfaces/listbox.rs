use console_engine::{screen::Screen, Color};

#[derive(Clone)]
pub struct ListBox {
    pub display: Vec<String>,
    pub page: usize,
    pub position: usize,
    pub screen: Screen,
    pub focused: bool,
    pub description: String,
}

impl ListBox {
    pub fn new<S: Into<String>>(w: u16, h: u16, focused: bool, description: S) -> Self {
        Self {
            display: Vec::new(),
            page: 0,
            position: 0,
            screen: Screen::new_empty(w as u32, h as u32),
            focused,
            description: description.into()
        }
    }

    pub fn get_page_count(&mut self) -> usize {
        self.display
            .chunks((self.screen.get_height() - 2) as usize)
            .len()
    }

    pub fn scroll_down(&mut self) {
        if self.position < self.screen.get_height() as usize - 2 {
            self.position += 1;
        } else {
            self.switch_page_up();
        }
    }

    pub fn scroll_up(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        } else {
            self.switch_page_down();
            self.position = self.screen.get_height() as usize - 2;
        }
    }

    pub fn switch_page_up(&mut self) {
        if self.page < self.get_page_count() {
            self.page += 1;
            self.position = 0;
        }
        self.screen.clear();
    }

    pub fn switch_page_down(&mut self) {
        if self.page > 0 {
            self.page -= 1;
        }
        self.screen.clear();
    }

    pub fn get_selected_idx(&mut self) -> usize {
       let pos = self.position + (self.page * self.screen.get_height() as usize);
       pos.checked_sub(1).unwrap_or(0)
    }

    pub fn remove(&mut self, value: String) {
        self.display.retain(|x| x == &value);
    }

    pub fn get_selected_str(&mut self) -> String {
        self.display[self.position].clone()
    }

    pub fn sel_idx_glob(&mut self, pos: usize) -> usize {
        pos + (self.page * self.screen.get_height() as usize)
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.position = 0;
        self.screen.clear();
        self.screen.resize(w as u32, h as u32);
    }

    pub fn draw(&mut self) -> &Screen {
        // drawing
        let splited_pags = self.display.chunks((self.screen.get_height() - 2) as usize);

        for (i, v) in &mut splited_pags.into_iter().enumerate() {
            if i == self.page {
                for (index, page) in v.into_iter().enumerate() {
                    if index == self.position {
                        self.screen
                            .print_fbg(0, index as i32, page, Color::Black, Color::White)
                    } else {
                        self.screen.print(0, index as i32, page);
                    }
                }
            }
        }
        &self.screen
    }
}
