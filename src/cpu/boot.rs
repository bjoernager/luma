/*
	Copyright 2021-2023 Gabriel Jensen.

	This file is part of Luma.

	Luma is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	Affero General Public License as published by
	the Free Software Foundation, either version 3
	of the License, or (at your option) any later
	version.

	Luma is distributed in the hope that it will be
	useful, but WITHOUT ANY WARRANTY; without even
	the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU
	Affero General Public License along with Luma.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::{log, log_status};
use crate::cpu::Cpu;
use crate::cpu_handle::CpuHandle;

use std::sync::atomic::Ordering;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

impl Cpu {
	pub fn boot(self) -> CpuHandle {
		eprintln!("starting emulation at {:#010X}", self.state.lock().unwrap().read_register(0xF).wrapping_sub(0x8));

		let state = self.state.clone();
		let dead  = self.dead.clone();

		let handle = spawn(move || { self.run() });

		return CpuHandle::new(
			state,
			dead,

			handle,
		);
	}

	fn run(mut self) {
		let run_timer = Instant::now();

		'main_loop: loop {
			if self.dead.load(Ordering::Relaxed) { break 'main_loop };

			let (instruction, predicate, cpsr) = self.fetch();
			match predicate.test(cpsr) {
				false => log_status!("skipping due to predicate ({})", predicate.code()),
				true  => self.execute(instruction),
			};
			self.r#continue();

			if cfg!(debug_assertions) { sleep(Duration::from_millis(125)) };

			self.cycle += 0x1;
		}

		let frequency = self.cycle as f64 / run_timer.elapsed().as_micros() as f64;
		eprintln!("emulated {} cycle(s) ({frequency:.9} MHz)", self.cycle);
	}
}
