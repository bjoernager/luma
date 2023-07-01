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
	Affero General Public License along with Luma. If not, 
	see <https://www.gnu.org/licenses/>. 
*/

use crate::luma::device::{Device, Log};

impl Device {
	pub fn store(&mut self, register: u8, base: u8, immediate: u16, u: bool, _b: bool, l: bool) {
		// TO-DO: Byte loads/stores.
		
		let offset = if u {
			0x0 + immediate as i32
		} else {
			0x0 - immediate as i32
		};

		let (address, _) = self.registers[base as usize].overflowing_add_signed(offset);

		if l { // Check the l flag.
			let value = self.read_word(address);
			self.registers[register as usize] = value;

			self.log(Log::Load(register, address, base, offset, value));
		} else {
			let value = self.registers[register as usize];
			self.write_word(address, value);

			self.log(Log::Store(address, register, base, offset, value));
		}
	}
}
