use shared::{HangmanChar, Game};

pub trait ClientGame {
    fn new(word_length: u8) -> Game;
    fn show_chars(&mut self, char_: char, positions: &[u8]);
}

impl ClientGame for Game {
    fn new(word_length: u8) -> Game {
        Game {
            word: (0..word_length).map(|_| HangmanChar { char_: '?', visible: false }).collect(),
            lives: 5
        }
    }
    
    fn show_chars(&mut self, char_: char, positions: &[u8]) {
        for &i in positions {
            self.word[i as usize].char_ = char_;
            self.word[i as usize].visible = true;
        }
    }
}
