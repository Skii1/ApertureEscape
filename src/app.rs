use std::collections::HashMap;

pub enum ScreenState {
    Default,
    View,
    Editing,
    Selecting,
}

pub struct App {
    pub key_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: ScreenState,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            pairs: HashMap::new(),
            current_screen: ScreenState::Default,
        }
    }
}