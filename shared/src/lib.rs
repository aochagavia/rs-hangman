use std::fmt;

pub struct HangmanChar {
    pub char_: char,
    pub visible: bool
}

impl fmt::Display for HangmanChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.visible {
            write!(f, "{}", self.char_)
        } else {
            write!(f, "_")
        }
    }
}

pub enum GameState {
    Playing,
    Victory,
    Defeat
}

pub struct Game {
    pub word: Vec<HangmanChar>,
    pub lives: u8,
}

impl Game {
    pub fn state(&self) -> GameState {
        if self.lives == 0 {
            GameState::Defeat
        } else if self.word.iter().all(|c| c.visible) {
            GameState::Victory
        } else {
            GameState::Playing
        }        
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "Lives: {}", self.lives));
        try!(write!(f, "Word: "));
        for c in &self.word {
            try!(write!(f, "{}", c))
        }
        writeln!(f, "")
    }
}
