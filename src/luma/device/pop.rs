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

use crate::luma::device::{Device, Log};

impl Device {
	pub fn thumb_pop(&mut self, list: u8, r: bool) {
		// Return true if branching.

		let ammount = list.count_ones() as u8 + r as u8;

		let mut address = self.registers[0xE];

		for index in 0x0..0x7 {
			let pop = (list >> index) & 0b00000001 != 0x0;

			if pop {
				let value = self.read_word(address);

				self.registers[index as usize] = value;
				self.log(Log::Pop, format!("r{index} => {address:#010X} ({value:#010X})"));

				address += 0x4;
			}
		}

		if r {
			let value = self.read_word(address);

			// We ignore the T flag.
			(self.registers[0xF], _) = (value & 0b11111111111111111111111111111110).overflowing_add(0x4);
			self.log(Log::Pop, format!("pc => {address:#010X}+4 ({value:#010X})"));
		}

		self.registers[0xE] = address - 0x4;
		self.log(Log::Pop, format!("sp => sp{:+} ({:#010X})", ammount * 0x4, self.registers[0xE]));
	}
}
