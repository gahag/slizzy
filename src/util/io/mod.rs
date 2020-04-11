pub mod fs;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Progress {
	pub total: Option<usize>,
	pub completed: usize,
}


impl Progress {
	pub fn percentage(&self) -> Option<f32> {
		let total = self.total? as f32;
		let completed = self.completed as f32;

		Some(completed / total)
	}
}
