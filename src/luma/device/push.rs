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
	pub fn thumb_push(&mut self, list: u8, r: bool) {
		let ammount = list.count_ones() as u8 + r as u8;

		let (start, _) = self.registers[0xE].overflowing_sub(ammount as u32 * 0x4);

		let mut address = start;

		for index in 0x0..0x7 {
			let push = (list >> index) & 0b00000001 != 0x0;

			if push {
				let value = self.registers[index as usize];

				self.write_word(address, value);
				self.log(Log::Push, format!("{address:#010X} => r{index} ({value:#010X})"));

				address += 0x4;
			}
		}

		if r {
			let value = self.registers[0xD];

			self.write_word(address, value);
			self.log(Log::Push, format!("{address:#010X} = lr ({value:#010X})"));
		}

		self.registers[0xE] = start;

		self.log(Log::Push, format!("sp => sp-{} ({start:#010X})", ammount * 0x4));
	}
}
