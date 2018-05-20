use std::time::{Instant,Duration};

#[derive(Debug,Copy,Clone)]
pub struct Limiter {
	wait_time: Duration,
	last_sleep: Instant,
	pub slow_function: fn(Duration),
}

impl Limiter {
	pub fn from_tps(tps: f64, slow_function: Option<fn(Duration)>) -> Self {
		let spt = 1.0 / tps;

		if spt.is_sign_negative() || !spt.is_normal() || spt.floor() > u64::max_value() as f64 { panic!("no"); }

		Self {
			wait_time: Duration::new(spt.floor() as u64, (spt.fract() * 1e9) as u32),
			last_sleep: Instant::now(),
			slow_function: slow_function.unwrap_or(default_slow_function),
		}
	}

	pub fn sleep(&mut self) {
		use std::thread::sleep;
		
		let e = self.last_sleep.elapsed();
		
		if let Some(t) = self.wait_time.checked_sub(e) {
			sleep(t);
		} else {
			(self.slow_function)(e-self.wait_time);
		}
		self.last_sleep = Instant::now();
	}
	
	pub fn reset(&mut self) {
		self.last_sleep = Instant::now();
	}
}

use std::cmp::{Eq,PartialEq,Ord,PartialOrd};

impl Eq for Limiter {}

impl PartialEq for Limiter {
	fn eq(&self, other: &Self) -> bool {
		self.wait_time.eq(&other.wait_time)
	}
}

use std::cmp::Ordering;

impl Ord for Limiter {
	fn cmp(&self, other: &Self) -> Ordering {
		self.wait_time.cmp(&other.wait_time)
	}
}

impl PartialOrd for Limiter {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.wait_time.partial_cmp(&other.wait_time)
	}
}

fn default_slow_function(slow: Duration) {
	println!("fell behind by {} seconds, {} nanoseconds",slow.as_secs(),slow.subsec_nanos());
}
