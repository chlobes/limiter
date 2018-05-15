use std::time::{Instant,Duration};

#[derive(Debug,Copy,Clone)]
pub struct Clock {
	wait_time: Duration,
	last_sleep: Instant,
}

impl Clock {
	pub fn from_tps(tps: f64) -> Self {
		let spt = 1.0 / tps;

		if spt.is_sign_negative() || !spt.is_normal() || spt.floor() > u64::max_value() as f64 { panic!("no"); }

		Self {
			wait_time: Duration::new(spt.floor() as u64, (spt.fract() * 1e9) as u32),
			last_sleep: Instant::now(),
		}
	}

	pub fn sleep(&mut self) -> Result<(), Duration> {
		use std::thread::sleep;
		
		let el = self.last_sleep.elapsed();
		
		let result = {
			if let Some(t) = self.wait_time.checked_sub(el) {
				sleep(t);
				Ok(())
			} else {
				Err(el-self.wait_time)
			}
		};
		self.last_sleep = Instant::now();
		result
	}
}

use std::cmp::{Eq,PartialEq,Ord,PartialOrd};

impl Eq for Clock {}

impl PartialEq for Clock {
	fn eq(&self, other: &Self) -> bool {
		self.wait_time.eq(&other.wait_time)
	}
}

use std::cmp::Ordering;

impl Ord for Clock {
	fn cmp(&self, other: &Self) -> Ordering {
		self.wait_time.cmp(&other.wait_time)
	}
}

impl PartialOrd for Clock {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.wait_time.partial_cmp(&other.wait_time)
	}
}
