use std::time::{Instant,Duration};

pub struct Limiter<'a> {
	wait_time: Duration,
	last_sleep: Instant,
	/// when `Limiter::sleep()` is called late, it calls this function with the amount of time it was late by.
	pub callback: Option<Box<dyn FnMut(Duration) + 'a>>,
}

impl<'a> Limiter<'a> {
	/// create a `Limiter` given a number of times per second.
	pub fn from_tps(tps: f64) -> Self {
		let spt = 1.0 / tps;
		assert!(!spt.is_sign_negative() && spt.is_normal() && spt * 1e9 <= u64::max_value() as f64,
			"Limiter::from_tps called with invalid value: {} (generates sleep time of {})",tps,spt);
		
		Self {
			wait_time: Duration::from_nanos((spt * 1e9) as u64),
			last_sleep: Instant::now(),
			callback: None,
		}
	}
	
	/// sleep until it's time for the next execution.
	pub fn sleep(&mut self) {
		let e = self.last_sleep.elapsed();
		if let Some(t) = self.wait_time.checked_sub(e) { //sleep was called in time
			std::thread::sleep(t);
		} else { //we are running late
			let t = e-self.wait_time;
			self.callback.as_mut().map(|f| f(t));
		}
		self.last_sleep = Instant::now();
	}
	
	/// restart the timer without sleeping.
	pub fn reset(&mut self) {
		self.last_sleep = Instant::now();
	}
	
	/// time left until next sleep, if any.
	pub fn time_left(&self) -> Option<Duration> {
		self.wait_time.checked_sub(self.last_sleep.elapsed())
	}
	
	/// time left as a fraction of total time, if any.
	pub fn frac_time_left(&self) -> Option<f64> {
		self.time_left().map(|t| t.as_nanos() as f64 / self.wait_time.as_nanos() as f64)
	}
}
