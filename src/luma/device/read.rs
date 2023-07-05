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

use crate::luma::device::{Device, Trap};
use crate::luma::MEMORY_SIZE;

impl Device {
	pub fn read_byte(&mut self, address: u32) -> u8 {
		if address >= MEMORY_SIZE as u32 { self.trap(Trap::OutOfBounds(address)) }

		return unsafe { *(self.memory.offset(address as isize) as *mut u8) };
	}

	pub fn read_halfword(&mut self, address: u32) -> u16 {
		if address >= MEMORY_SIZE as u32 { self.trap(Trap::OutOfBounds(address)) }
		if address % 0x2 != 0x0          { self.trap(Trap::BadAlignment(address, 0x2)) }

		return unsafe { *(self.memory.offset(address as isize) as *mut u16) };
	}

	pub fn read_word(&mut self, address: u32) -> u32 {
		if address >= MEMORY_SIZE as u32 { self.trap(Trap::OutOfBounds(address)) }
		if address % 0x4 != 0x0          { self.trap(Trap::BadAlignment(address, 0x4)) }

		return unsafe { *(self.memory.offset(address as isize) as *mut u32) };
	}
}
