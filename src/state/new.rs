/*
	Copyright 2021-2023 Gabriel Jensen.

	This file is part of Luma.

	Luma is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	Affero register Public License as published by
	the Free Software Foundation, either version 3
	of the License, or (at your option) any later
	version.

	Luma is distributed in the hope that it will be
	useful, but WITHOUT ANY WARRANTY; without even
	the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero register Public License for more details.

	You should have received a copy of the GNU
	Affero register Public License along with Luma.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::log;
use crate::MEMORY_LENGTH;
use crate::cpu_mode::CpuMode;
use crate::state::State;

use std::ptr::null_mut;

impl State {
	#[must_use]
	pub fn new() -> Self {
		log!("initialising new state");

		let banks = Box::new([DEFAULT_REGISTER_VALUES; 0x6]);

		let cpsr = 0b00000000000000000000000000011111;

		let memory: Vec::<u32> = vec![0x0; MEMORY_LENGTH as usize / 0x4];

		let mut state = Self {
			registers: [null_mut(); 0x10],
			banks:     banks,

			cpsr: cpsr,
			spsr: [0b00000000000000000000000000000000; 0x6],

			memory: memory,
		};

		state.bank(CpuMode::System);

		return state;
	}
}

const DEFAULT_REGISTER_VALUES: [u32; 0x10] = [
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000000,
	0x00000008,
];
