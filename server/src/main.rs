extern crate rand;
extern crate shared;

mod server_game;

use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

use rand::Rng;
use server_game::ServerGame;
use shared::{GameState, Game, read_message, send_message};
use shared::guess::Guess;
use shared::guess_result::GuessResult;
use shared::word_length::WordLength;

const ADDR: &'static str = "127.0.0.1:8181";
const WORDS: &'static [&'static str] = &["rustacean", "safety", "concurrency", "speed"];

fn main() {
    let listener = TcpListener::bind(ADDR).unwrap();
    println!("Server listening on {}", ADDR);

    for stream in listener.incoming() {
        thread::spawn(move || {
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
    let mut length = WordLength::new();
    length.set_length(game.word.len() as u32);
    send_message(length, &mut stream);

    // Receive the guesses of the player
    while let GameState::Playing = game.state() {
        let guess: Guess = read_message(&mut stream).unwrap();
        let char_ = guess.get_character().chars().next().unwrap();

        let indices = game.guess(char_);
        let mut result = GuessResult::new();
        if indices.len() > 0 {
            result.set_correct(true);
            result.set_indices(indices);
        } else {
            result.set_correct(false);
            result.set_indices(Vec::new());
        }
        send_message(result, &mut stream);
    }
}
