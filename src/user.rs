use api;

pub trait Bot {
	fn update(&mut self, state: &api::State) -> api::Action;
}