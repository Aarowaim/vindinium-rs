pub mod api;
pub mod http;
pub mod user;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate maplit;

use api::Session;
use http::Client;
use user::Bot;

let key = "__".to_string();

struct RandomBot;

impl Bot for RandomBot {
	fn update(&mut self, state: &api::State) -> api::Action {
		api::Action::South
	}
}

fn main() {
	let mut test = Client::new("http://vindinium.org".to_string(), 
		key, ConstraintBot{});
	let mut content = test.open_training(300, String::from("m1"));
	while !content.game.finished {
		content = test.submit_action(content);
		println!("{}", content.game.board.to_string());
	}

	println!("{}", content.game.board);
	println!("{}", content.viewUrl);
}