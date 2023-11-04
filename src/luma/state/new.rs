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

use crate::luma::MEMORY_LENGTH;
use crate::luma::state::State;

impl State {
	#[must_use]
	pub fn new() -> Self {
		let registers: [u32; 0x10] = [
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
			0x08000008,
		];

		let cpsr = 0b00000000000000000000000000001111;

		let memory: Vec::<u32> = vec![0x0; MEMORY_LENGTH as usize / 0x4];

		return Self {
			registers: registers,
			cpsr:      cpsr,

			memory: memory,
		};
	}
}
