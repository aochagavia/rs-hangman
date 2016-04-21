extern crate protobuf;

pub mod guess;
pub mod guess_result;
pub mod word_length;

use std::fmt;
use std::io::{Cursor, Read, Write};

use protobuf::{Message, MessageStatic, ProtobufResult};

// Send a message through the given writer
pub fn send_message<M, W>(message: M, writer: &mut W)
    where M: Message, W: Write
{
    // FIXME: is there a more efficient way to do this?
    // Write message to a vec
    let bytes = message.write_to_bytes().expect("Unable to write bytes");
    
    // Remember that the length of the message is stored in a single byte
    assert!(bytes.len() < 256);
    
    // Write the amount of bytes first
    writer.write_all(&[bytes.len() as u8]).expect("Unable to write byte amount");
    
    // Write the rest
    writer.write_all(&bytes).expect("Unable to write all bytes");
    writer.flush().unwrap();
}

// Read a message from the given reader
pub fn read_message<M, R>(reader: &mut R) -> ProtobufResult<M>
    where M: Message + MessageStatic, R: Read
{
    // Read the number of incoming bytes
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).expect("Unable to read to buf");
    let bytes = buf[0] as usize;
    
    // Read the message into a buffer
    // FIXME: there is a way to read without copying (CodingInputStream)
    // See https://developers.google.com/protocol-buffers/docs/techniques#streaming
    let mut big_buf: Vec<u8> = vec![0; bytes];
    reader.read_exact(&mut big_buf).expect("Unable to read to big_buf");

    // Read the rest of the message
    protobuf::parse_from_reader::<M>(&mut Cursor::new(big_buf))
}

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
