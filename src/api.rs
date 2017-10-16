use std::fmt;

// declarations
pub trait Session {
	fn new(&mut self, key: String);

	fn open_training(&self, turns: usize, map_id: String) -> State;
	fn open_arena(&self);

	fn submit_action(&mut self, dir: State) -> State;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
	pub game: Game,
	pub hero: Hero,
	pub token: String,
	pub viewUrl: String,
	pub playUrl: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
	pub id: String,
	pub turn: usize,
	pub maxTurns: usize,
	pub heroes: Vec<Hero>,
	pub board: Board,
	pub finished: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hero {
	pub id: usize,
	pub name: String,
	pub userId: Option<String>,
	pub elo: Option<usize>,
	pub pos: Point,
	pub lastDir: Option<Action>,
	pub life: usize,
	pub gold: usize,
	pub mineCount: usize,
	pub spawnPos: Point,
	pub crashed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
	pub size: usize,
	pub tiles: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
	pub x: usize,
	pub y: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Action {
		North,
	West, Stay, East,
		South,
}

// implementations
fn partition(input: &String, length: usize) -> String {
	input
		.chars()
		.collect::<Vec<_>>()
		.chunks(length)
		.collect::<Vec<_>>()
		.join(&'\n')
		.into_iter()
		.collect()
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", partition(&self.tiles, 2 * self.size))
	}
}

impl fmt::Display for Hero {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}: {}\nHP: {}\tGOLD: {}+({})", 
			self.id, self.name, self.life, self.gold, self.mineCount)
	}
}

impl Game {
	pub fn leaderboard(&self) -> String {
		format!("{}", self.heroes
			.iter()
			.map(|h| h.to_string())
			.collect::<Vec<String>>()
			.join("\n"))
	}
}