#![feature(io, net, std_misc)]

extern crate rand;
extern crate shared;
mod server_game;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::Thread;

use rand::Rng;
use server_game::ServerGame;
use shared::{GameState, Game};

const ADDR: &'static str = "127.0.0.1:8181";
const WORDS: &'static [&'static str] = &["rustacean", "safety", "concurrency", "speed"];

fn main() {
    let listener = TcpListener::bind(ADDR).unwrap();
    println!("Server listening on {}", ADDR);

    for stream in listener.incoming() {
        Thread::spawn(move || {
            handle_client(stream.unwrap());
            println!("Player logged out");
        });
    }
}

fn handle_client(mut stream: TcpStream) {
    let word = rand::thread_rng().choose(WORDS).unwrap();
    let mut game = <Game as ServerGame>::new(word);
    println!("Player logged in. Assigned word '{}'", word);

    // Send word length
    stream.write(&[game.word.len() as u8]).unwrap();

    // Receive the guesses of the player
    // It will always be a character, so 16 bytes is more than enough
    let mut buffer = [0u8; 16];
    while let GameState::Playing = game.state() {
        let read_bytes = stream.read(&mut buffer).unwrap();
        let char_ = String::from_utf8(buffer[..read_bytes].to_vec()).unwrap().chars()
                                                                    .next().unwrap();

        let indices = game.guess(char_);
        if indices.len() > 0 {
            stream.write(&indices).unwrap();
        } else {
            stream.write(&[255]).unwrap();
        }
    }
}