#![feature(io, net)]

extern crate shared;
mod client_game;

use std::io::{Read, Write};
use std::old_io;
use std::net::TcpStream;

use client_game::ClientGame;
use shared::{GameState, Game};

const ADDR: &'static str = "127.0.0.1:8181";

fn main() {
    println!("Connecting to {}", ADDR);
    let mut stream = TcpStream::connect(ADDR).unwrap();
    
    // Connection established, retrieve word length
    let mut buffer = [0u8; 255];
    assert_eq!(stream.read(&mut buffer), Ok(1));
    let mut game = <Game as ClientGame>::new(buffer[0]);
    
    // The game loop
    let mut reader = old_io::stdin();
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
        let word = reader.read_line().unwrap();
        let bytes: Vec<_> = word.trim_right().bytes().take(16).collect();
        stream.write(&bytes).unwrap();
        
        // The server will send a list of indices or one 255 character
        let length = stream.read(&mut buffer).unwrap();
        if buffer[0] == 255 {
            game.lives -= 1;
        } else {
            game.show_chars(word.chars().next().unwrap(), &buffer[..length]);
        }
    }
}
