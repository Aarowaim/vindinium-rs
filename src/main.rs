pub mod api;
pub mod http;
pub mod user;

#[cfg(test)]
mod tests;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate maplit;

extern crate rand;
use rand::Rng;

extern crate multiqueue;
extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;

use std::fs::File;
use std::io::prelude::*;

use api::*;
use http::Client;
use user::Bot;

struct RandomBot;

impl Bot for RandomBot {
	fn update(&mut self, state: &State) -> Action {
		let actions = vec![Action::North, Action::South, Action::West, Action::East];
		rand::thread_rng().choose(&actions)
			.expect("No values to choose in [T] provided.").clone()
	}
}

enum Message {
	Json(String),
	GameState(State),
	Output(String),
	Terminate,
}

fn main() {
	let key_file = "key.txt";
	let mut f = File::open(key_file)
		.expect(format!("No key file found in current directory. Please create '{}'", key_file).as_str());

	let mut key = String::new();
	f.read_to_string(&mut key)
		.expect(format!("Could not read '{}'.", key_file).as_str());
	key.trim();

	let mut client = Client::new("http://vindinium.org".to_string(), 
		key, RandomBot{});

	let mut content = client.open_training(300, String::from("m1"));

	while !content.game.finished {
		content = client.submit_action(content);
		println!("\n{}", content.viewUrl);
		println!("{}", content.game.leaderboard());
		println!("{}", content.game.board);
	}
}

//TODO: Use `Service` to generate futures containing "Hello" or something
/*fn my_test_sender(send: multiqueue::MPMCFutSender<String>, recv: multiqueue::MPMCFutReceiver<String>) {
	std::thread::spawn(|| {
	});
}*/