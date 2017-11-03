pub mod api;
pub mod http;
pub mod user;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate maplit;

extern crate rand;
use rand::Rng;

extern crate chan;
extern crate futures;
extern crate tokio_core;

use std::fs::File;
use std::io::prelude::*;

use api::*;
use http::Client;
use user::Bot;

struct RandomBot;

impl Bot for RandomBot {
	fn update(&mut self, state: &State) -> Action {
		let actions = vec![Action::North, Action::South, Action::West, Action::East];
		rand::thread_rng().choose(&actions).unwrap().clone()
	}
}

enum Message {
	Json(String),
	GameState(State),
	Output(String),
	Terminate,
}

struct RX<T>(chan::Receiver<T>);
impl<T> futures::stream::Stream for RX<T> {
	type Item = T;
	type Error = Box<std::error::Error>;

	fn poll(&mut self) -> futures::Poll<Option<Self::Item>, Self::Error> {
		let &mut RX(ref receiver) = self;
		let item = receiver.recv();
		match item {
			Some(value) => Ok(futures::Async::Ready(Some(value))),
			None => Ok(futures::Async::NotReady),
		}
		
	}

}

fn main() {
	let key_file = "key.txt";
	let mut f = File::open(key_file)
		.expect(format!("No key file found in current directory. Please create '{}'", key_file).as_str());

	let mut key = String::new();
	f.read_to_string(&mut key)
		.expect(format!("Could not read '{}'.", key_file).as_str());

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

use futures::{Stream, Future};

fn my_test() {
	let mut core = tokio_core::reactor::Core::new().unwrap();
	let handle = core.handle();

	let (tx, rx) = chan::async::<String>();

	tx.send("Hello".to_string());
	let incoming = RX(rx).for_each(|s| {
		println!("Result: {}", s);
		Ok(())
	});

	core.run(incoming).unwrap()
}