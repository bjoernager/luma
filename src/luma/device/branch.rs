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

use crate::luma::device::{Branch, Device};

impl Device {
	pub fn branch(&mut self, kind: Branch) {
		match kind {
			Branch::Offset(offset, l) => {
				if l { // Check the l flag.
					// Store the address of the following instruction 
					// in r14 (lr).

					let pc_offset: u32 = match self.thumb() {
						false => 0x4,
						true  => 0x2,
					};

					self.registers[0xE] = self.registers[0xF] - pc_offset;
		
					self.log("link", format!("r14 => r15-{pc_offset}={:#010X}", self.registers[0xE]));
				}

				// Add the offset to r15 (pc).

				let (address, _) = self.registers[0xF].overflowing_add_signed(offset);
	
				// Add extra offset to move to the new fetch 
				// instruction.
				let pc_offset = match self.thumb() {
					false => 0x8,
					true  => 0x4,
				};

				self.registers[0xF] = address + pc_offset;
				
				self.log("branch", format!("r15 => r15{offset:+}+{pc_offset} ({:#010X})", self.registers[0xF]));
			},
			Branch::Register(register) => {
				// Use the address stored in 'register' as the new 
				// value in r15 (pc).

				let value = self.registers[register as usize];

				let t = value & 0b00000000000000000000000000000001 != 0x0;

				self.cpsr = self.cpsr & 0b11111111111111111111111111011111 | (t as u32) << 0x5;
				self.exchange(t);

				let address = value & 0b11111111111111111111111111111110;

				// Add extra offset to move to the new fetch 
				// instruction.
				let pc_offset: u32 = match t {
					false => 0x8,
					true  => 0x4,
				};

				self.registers[0xF] = address + pc_offset;

				self.log("branch", format!("r15 => r{register}{pc_offset:+} ({:#010X})", self.registers[0xF]));
			},
		}
	}
}
