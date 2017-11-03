extern crate serde;
extern crate serde_json;

extern crate reqwest;

use std::collections::HashMap;
use std::io::Read;

use user::Bot;
use api::{State, Action, Session};

fn reqwest_url(s: String) -> reqwest::Url {
	reqwest::Url::parse(&s.as_str()).expect(format!("Failed to connect to {}", &s).as_str())
}

pub struct Client<B: Bot> {
	client: reqwest::Client,
	base_uri: reqwest::Url,

	key: String,
	bot: B,
}

impl <B: Bot> Client<B> {

	pub fn new(base_uri: String, key: String, bot: B) -> Client<B> {
		Client { 
			client: reqwest::Client::new(),
			base_uri: reqwest_url(base_uri),
			key: key,
			bot: bot,
		}
	}

	pub fn post(&self, path: reqwest::Url, args: HashMap<&str, String>) -> State {

		let mut res = self.client.post(path)
			.form(&args)
			.send()
			.expect("Failed to post to vindinium server");

		let mut content = String::new();
		res.read_to_string(&mut content).expect("Received no response from the vindinium server");
		serde_json::from_str(content.as_str()).expect("Failed to parse json response")
	}
}

impl <B: Bot> Session for Client<B> {
	fn new(&mut self, key: String) {
		self.key = key
	}

	fn open_training(&self, turns: usize, map_id: String) -> State {
		let args = hashmap!{
			"key" => self.key.clone(),
			"turns" => turns.to_string().clone(),
			"map" => map_id.clone(),
		};

		let path = self.base_uri.join("/api/training").expect("Failed to join urls");
		self.post(path, args)
	}

	fn open_arena(&self) {
		let args = hashmap!{
			"key" => self.key.clone(),
		};

		let path = self.base_uri.join("/api/arena").expect("Failed to join urls");
		self.post(path, args);
	}

	fn submit_action(&mut self, state: State) -> State {
		let action = self.bot.update(&state);

		let args = hashmap!{
			"key" => self.key.clone(),
			"dir" => format!("{:?}", action),
		};
		let path = reqwest_url(state.playUrl);
		self.post(path, args)
	}
}