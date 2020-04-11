use trigram::similarity;

use super::Sim;


pub fn str(s1: &str, s2: &str) -> Sim {
	let sim = similarity(s1, s2) * 100.0;

	log::info!("similarity between '{}' and '{}': {}", s1, s2, sim);

	Sim(sim as u8)
}
