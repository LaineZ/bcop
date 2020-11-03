use console_engine::{screen::Screen, Color};

#[derive(Clone)]
/// A listbox user interface
pub struct ListBox {
    /// Listbox items
    pub display: Vec<String>,
    /// Current displaying page
    pub page: usize,
    /// Current on-page cursor position. you can safely read/write this value
    pub position: usize,
    /// ListBox Screen struct
    pub screen: Screen,
    /// In focus whether listbox?
    pub focused: bool,
    /// Listbox bottom name and description, keys, usage
    pub description: String,
}

impl ListBox {
    /// Creates listbox
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

    /// Gets listbox page count
    pub fn get_page_count(&mut self) -> usize {
        self.display
            .chunks((self.screen.get_height() - 2) as usize)
            .len()
    }

    /// Scrolls listbox down by 1 item. if reaches end of current page - switches to another
    pub fn scroll_down(&mut self) {
        if self.position < self.screen.get_height() as usize - 2 {
            self.position += 1;
        } else {
            self.switch_page_up();
        }
    }

    /// Scrolls listbox up by 1 item. if reaches start of current page - switches to another
    pub fn scroll_up(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        } else {
            self.switch_page_down();
            self.position = self.screen.get_height() as usize - 2;
        }
    }

    /// Scrolls listbox up by 1 page
    pub fn switch_page_up(&mut self) {
        if self.page < self.get_page_count() {
            self.page += 1;
            self.position = 0;
        }
        self.screen.clear();
    }

    /// Scrolls listbox down by 1 page
    pub fn switch_page_down(&mut self) {
        if self.page > 0 {
            self.page -= 1;
        }
        self.screen.clear();
    }

    /// Gets selected listbox index
    pub fn get_selected_idx(&mut self) -> usize {
       let pos = self.position + (self.page * self.screen.get_height() as usize);
       pos.checked_sub(1).unwrap_or(0)
    }

    /// Removes listbox items by value
    pub fn remove(&mut self, value: String) {
        self.display.retain(|x| x == &value);
    }

    /// Gets current selected String in listbox
    pub fn get_selected_str(&mut self) -> String {
        self.display[self.position].clone()
    }

    /// Sets listbox position and sets the needed page
    pub fn sel_idx_glob(&mut self, pos: usize) -> usize {
        pos + (self.page * self.screen.get_height() as usize)
    }

    /// Resizes listbox to specified dimensions
    pub fn resize(&mut self, w: u16, h: u16) {
        self.position = 0;
        self.screen.clear();
        self.screen.resize(w as u32, h as u32);
    }

    /// Draws listbox. For more stability recommended to use in print_screen function
    pub fn draw(&mut self) -> &Screen {
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
