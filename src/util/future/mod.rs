pub mod abortable;

use std::future::Future;


pub async fn retry<F, O, E>(
	fetch: impl Fn() -> F,
	retry: impl Fn(&E) -> bool,
	report: impl Fn(usize, E) -> E,
	limit: Option<usize>
) -> F::Output
where
	F: Future<Output = Result<O, E>>,
	E: std::error::Error,
{
	let mut attempt: usize = 1;

	loop {
		let result = fetch().await;

		match result {
			Err(error) if retry(&error) => {
				let error = report(attempt, error);

				attempt += 1;

				let exceeded = limit
					.map(|l| attempt > l)
					.unwrap_or(false);

				if exceeded {
					return Err(error);
				}
			},

			other => return other,
		}
	};
}
