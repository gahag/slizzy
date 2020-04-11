use std::iter::{Extend, FromIterator};

use thiserror::Error;

use derive_more::Deref;


#[macro_export]
macro_rules! report_wrapped {
	($wraped: expr, $report: expr, $unwrap: pat => $error: expr) => {
		{
			let status = $wraped;

			$report(&status);

			match status {
				$unwrap => $error,
				other => panic!("wrapper is not what was constructed: {:?}", other)
			}
		}
	};
}


pub fn anyhow_result<T, E>(result: Result<T, E>) -> anyhow::Result<T>
where
	E: std::error::Error + std::marker::Send + std::marker::Sync + 'static
{
	result.map_err(anyhow::Error::new)
}


#[derive(Debug, Error, Deref)]
pub struct AggregateError(
	// We use vec to allow extending, as a future::Stream requires extending to collect.
	pub Vec<anyhow::Error>
);


impl std::fmt::Display for AggregateError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.write_str("aggregate error:\n")?;

		for error in self.0.iter() {
			writeln!(f, "  {}", error)?
		}

		Ok(())
	}
}


impl Default for AggregateError {
	fn default() -> Self {
		Self(
			Default::default()
		)
	}
}


impl Extend<anyhow::Error> for AggregateError {
	fn extend<I>(&mut self, iter: I)
	where
		I: IntoIterator<Item = anyhow::Error>
	{
		self.0.extend(iter);
	}
}


impl FromIterator<anyhow::Error> for AggregateError {
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = anyhow::Error>
	{
		let iter = iter.into_iter();

		Self(
			iter.collect()
		)
	}
}
