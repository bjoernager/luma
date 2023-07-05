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
	pub fn arm_store(&mut self, destination: u8, base: u8, immediate: u16, u: bool, _b: bool, l: bool) {
		// TO-DO: Byte loads/stores.

		// The U flag determins the sign of the offset
		// (set = unsigned).
		let offset = if u {
			0x0 + immediate as i32
		} else {
			0x0 - immediate as i32
		};

		let (address, _) = self.registers[base as usize].overflowing_add_signed(offset);

		if l { // Check the L flag.
			// If the L flag is set, we perform a memory-to-
			// destination load instead.

			let value = self.read_word(address);
			self.registers[destination as usize] = value;

			self.log(Log::Load, format!("r{destination} => r{base}{offset:+}={address:#010X} ({value:#010X})"));
		} else {
			// Otherwise, we perform a destination-to-memory
			// store.

			let value = self.registers[destination as usize];
			self.write_word(address, value);

			self.log(Log::Store, format!("r{base}{offset:+}={address:#010X} => r{destination} ({value:#010X})"));
		}
	}

	pub fn thumb_store_halfword_immediate(&mut self, source: u8, base: u8, immediate: u8) {
		// Load halfword from memory using immediate offset.

		let base_value = self.registers[base as usize];

		let offset = (immediate as u32) << 0x1;

		let (address, _) = base_value.overflowing_add(offset);

		let value = (self.registers[source as usize] & 0b00000000000000001111111111111111) as u16;
		self.write_halfword(address, value);

		self.log(Log::Store, format!("r{source} => r{base}{immediate:+}={address:#010X} ({value:#06X})"));
	}

}
