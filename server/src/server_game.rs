use shared::{HangmanChar, Game};

pub trait ServerGame {
    fn new(word: &str) -> Game;
    fn guess(&mut self, char_: char) -> Vec<u8>;
}

impl ServerGame for Game {
    fn new(word: &str) -> Game {
        let w = word.chars().map(|c| HangmanChar { char_: c, visible: false }).collect();
        Game { word: w, lives: 5 }
    }

    fn guess(&mut self, char_: char) -> Vec<u8> {
        let mut indices = Vec::new();
        for (i, h_char) in self.word.iter_mut().enumerate() {
            if h_char.char_ == char_ {
                h_char.visible = true;
                indices.push(i as u8);
            }
        }

        if indices.len() == 0 {
            self.lives -= 1;
        }

        indices
    }
}
