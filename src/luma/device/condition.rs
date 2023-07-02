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

use crate::luma::device::Device;

impl Device {
	pub fn condition(&self, condition: u8) -> bool {
		return match condition {
			0x0 => self.cpsr & 0b01000000000000000000000000000000 != 0x00,
			0x1 => self.cpsr & 0b01000000000000000000000000000000 == 0x00,
			0x2 => self.cpsr & 0b00100000000000000000000000000000 != 0x00,
			0x3 => self.cpsr & 0b00100000000000000000000000000000 == 0x00,
			0x4 => self.cpsr & 0b10000000000000000000000000000000 != 0x00,
			0x5 => self.cpsr & 0b10000000000000000000000000000000 == 0x00,
			0x6 => self.cpsr & 0b00010000000000000000000000000000 != 0x00,
			0x7 => self.cpsr & 0b00010000000000000000000000000000 == 0x00,
			0x8 => self.cpsr & 0b00100000000000000000000000000000 != 0x00 && self.cpsr & 0b01000000000000000000000000000000 == 0x00,
			0x9 => self.cpsr & 0b00100000000000000000000000000000 == 0x00 && self.cpsr & 0b01000000000000000000000000000000 != 0x00,
			0xA => self.cpsr & 0b00010000000000000000000000000000 >> 0x1C == self.cpsr & 0b10000000000000000000000000000000 >> 0x1F,
			0xB => self.cpsr & 0b00010000000000000000000000000000 >> 0x1C != self.cpsr & 0b10000000000000000000000000000000 >> 0x1F,
			0xC => self.cpsr & 0b01000000000000000000000000000000 == 0x00 && self.cpsr & 0b00010000000000000000000000000000 >> 0x1C == self.cpsr & 0b10000000000000000000000000000000 >> 0x1F,
			0xD => self.cpsr & 0b01000000000000000000000000000000 != 0x00 || self.cpsr & 0b00010000000000000000000000000000 >> 0x1C != self.cpsr & 0b10000000000000000000000000000000 >> 0x1F,
			0xE => true,
			0xF => false, // Unpredictable, but we ignore it.
			_   => unreachable!(),
		}
	}
}
