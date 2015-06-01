extern crate shared;
mod client_game;

use std::io::{self, Read, Write};
use std::net::TcpStream;

use client_game::ClientGame;
use shared::{GameState, Game};

const ADDR: &'static str = "127.0.0.1:8181";

fn main() {
    println!("Connecting to {}", ADDR);
    let mut stream = TcpStream::connect(ADDR).unwrap();

    // Connection established, retrieve word length
    let buffer = &mut [0u8; 255];
    assert_eq!(stream.read(buffer).ok(), Some(1));
    let mut game = <Game as ClientGame>::new(buffer[0]);

    // The game loop
    let mut line_buffer = String::new();
    let mut reader = io::stdin();
    loop {
        match game.state() {
            GameState::Victory => {
                println!("Congratulations! You won!");
                return;
            }
            GameState::Defeat => {
                println!("Sorry, you lost! Better luck next time!");
                return;
            }
            _ => {}
        }

        // Show the current state
        println!("{}", game);
        print!("Guess a letter: ");

        // Read next char and send it
        line_buffer.clear();
        reader.read_line(&mut line_buffer).unwrap();
        let bytes: Vec<_> = line_buffer.trim_right().bytes().take(16).collect();
        stream.write(&bytes).unwrap();

        // The server will send a list of indices or one 255 character
        let length = stream.read(buffer).unwrap();
        if buffer[0] == 255 {
            println!("Received only one number");
            game.lives -= 1;
        } else {
            println!("received data: {:?}", &buffer[..length]);
            game.show_chars(line_buffer.chars().next().unwrap(), &buffer[..length]);
        }

    }
}
