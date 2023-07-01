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

use crate::luma::device::{Branch, Device, Log};

impl Device {
	pub fn branch(&mut self, kind: Branch) {
		match kind {
			Branch::Offset(  offset,   l) => {
				if l { // Check the l flag.
					self.registers[0xE] = self.registers[0xF] - 0x4;
		
					self.log(Log::Link(self.registers[0xE]));
				}
		
				(self.registers[0xF], _) = self.registers[0xF].overflowing_add_signed(offset + 0x8); // Add extra eight to move to the new fetch instruction.
		
				self.log(Log::BranchOffset(offset, self.registers[0xF] - 0x8));
			},
			Branch::Register(register) => {
				let value = self.registers[register as usize];

				self.cpsr ^= (value & 0b00000000000000000000000000000001) << 0x5;

				let address = value & 0b11111111111111111111111111111110;
				self.registers[0xF] = address + 0x8;

				if value & 0b00000000000000000000000000000001 != 0x0 { eprintln!("switching to thumb") }

				self.log(Log::BranchRegister(register, address));
			},
		}
	}
}
