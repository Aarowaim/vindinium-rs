pub mod api;
pub mod http;
pub mod user;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate maplit;

use std::fs::File;
use std::io::prelude::*;

use api::Session;
use http::Client;
use user::Bot;

struct RandomBot;

impl Bot for RandomBot {
	fn update(&mut self, state: &api::State) -> api::Action {
		api::Action::South
	}
}

fn main() {
	let key_file = "key.txt";
	let mut f = File::open(key_file)
		.expect(format!("No key file found in current directory. Please create '{}'", key_file).as_str());

	let mut key = String::new();
	f.read_to_string(&mut key)
		.expect(format!("Could not read '{}'.", key_file).as_str());

	let mut test = Client::new("http://vindinium.org".to_string(), 
		key, RandomBot{});
	let mut content = test.open_training(300, String::from("m1"));
	while !content.game.finished {
		content = test.submit_action(content);
		println!("{}", content.game.board.to_string());
	}

	println!("{}", content.game.board);
	println!("{}", content.viewUrl);
}