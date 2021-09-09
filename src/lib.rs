use std::time::{Instant,Duration};

pub struct Limiter<'a> {
	wait_time: Duration,
	last_sleep: Instant,
	pub callback: Option<Box<dyn FnMut(Duration) + 'a>>, //when running slow, this function is called with the amount of time the call to Limiter::sleep() was late by
}

impl<'a> Limiter<'a> {
	pub fn from_tps(tps: f64) -> Self {
		let spt = 1.0 / tps;
		
		if spt.is_sign_negative() || !spt.is_normal() || spt.floor() > u64::max_value() as f64 { panic!("no {}",spt); }
		
		Self {
			wait_time: Duration::new(spt.floor() as u64, (spt.fract() * 1e9) as u32),
			last_sleep: Instant::now(),
			callback: None,
		}
	}

	pub fn sleep(&mut self) {
		let e = self.last_sleep.elapsed();
		
		if let Some(t) = self.wait_time.checked_sub(e) {
			std::thread::sleep(t);
		} else {
			let t = e-self.wait_time;
			self.callback.as_mut().map(|f| f(t));
		}
		self.last_sleep = Instant::now();
	}
	
	pub fn reset(&mut self) {
		self.last_sleep = Instant::now();
	}

	pub fn time_left(&self) -> Option<Duration> {
		self.wait_time.checked_sub(self.last_sleep.elapsed())
	}
	
	pub fn frac_time_left(&self) -> Option<f64> {
		self.time_left().map(|t| ((t.as_secs() as f64 * 10f64.powi(9)) + t.subsec_nanos() as f64) / ((self.wait_time.as_secs() as f64 * 10f64.powi(9)) + (self.wait_time.subsec_nanos() as f64)))
	}
}
