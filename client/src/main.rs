extern crate shared;
mod client_game;

use std::io;
use std::net::TcpStream;

use client_game::ClientGame;
use shared::{GameState, Game, read_message, send_message};
use shared::guess::Guess;
use shared::guess_result::GuessResult;
use shared::word_length::WordLength;

const ADDR: &'static str = "127.0.0.1:8181";

fn main() {
    println!("Connecting to {}", ADDR);
    let mut stream = TcpStream::connect(ADDR).unwrap();

    // Connection established, retrieve word length
    let length: WordLength = read_message(&mut stream).unwrap();
    let mut game = <Game as ClientGame>::new(length.get_length() as u8);

    // The game loop
    let mut line_buffer = String::new();
    let reader = io::stdin();
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
        let letter = line_buffer.chars().next().unwrap();
        let mut guess = Guess::new();
        guess.set_character(letter.to_string());
        send_message(guess, &mut stream);

        let result: GuessResult = read_message(&mut stream).unwrap();
        if !result.get_correct() {
            println!("Incorrect guess!");
            game.lives -= 1;
        } else {
            game.show_chars(letter, result.get_indices());
        }

    }
}
