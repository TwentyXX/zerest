use rand::seq::SliceRandom;

use super::App;

impl App {
	pub(crate) fn update_word(&mut self) {
		const LOREM_WORDS: &[&str] = &[
			"Lorem",
			"ipsum",
			"dolor",
			"sit",
			"amet",
			"consectetur",
			"adipiscing",
			"elit",
			"sed",
			"do",
			"eiusmod",
			"tempor",
			"incididunt",
			"ut",
			"labore",
			"et",
			"dolore",
			"magna",
			"aliqua",
		];

		if let Some(word) = LOREM_WORDS.choose(&mut rand::thread_rng()) {
			self.current_word += word;
			self.current_word += " ";
		}
	}
}
