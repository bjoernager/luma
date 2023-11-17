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

use crate::cpu::Cpu;

use std::thread::sleep;
use std::time::{Duration, Instant};

impl Cpu {
	pub fn sync_cycle(&self, cycle_start: Instant) {
		// 16*1024*1024 Hz, 59.604644775 ns

		const CYCLE_DURATION: f64 = 0.000000059604644775;
		let cycle_duration        = Duration::from_secs_f64(CYCLE_DURATION);

		let remaining = match cycle_duration.checked_sub(cycle_start.elapsed()) {
			Some(value) => value,
			None        => Duration::from_secs(0x0),
		};

		sleep(remaining);
	}
}
