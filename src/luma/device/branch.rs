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
	pub fn arm_branch(&mut self, offset: i32, l: bool) {
		// Add the offset to r15 (pc). Conditionally link.

		if l { self.arm_link() }

		let (address, _) = self.registers[0xF].overflowing_add_signed(offset);

		// Add extra offset to move to the new fetch
		// instruction.
		self.registers[0xF] = address + 0x8;

		self.log(Log::Branch, format!("pc => pc{offset:+}+8 ({:#010X})", self.registers[0xF]));
	}

	pub fn arm_branch_exchange(&mut self, register: u8) {
		// Use the address stored in 'register' as the new
		// value in r15 (pc).

		let value = self.registers[register as usize];

		let t = value & 0b00000000000000000000000000000001 != 0x0;

		self.cpsr = self.cpsr & 0b11111111111111111111111111011111 | (t as u32) << 0x5;
		self.exchange(t);

		// Add extra offset to move to the new fetch
		// instruction.
		let pc_offset: u32 = match t {
			false => 0x8,
			true  => 0x4,
		};

		self.registers[0xF] = (value & 0b11111111111111111111111111111110) + pc_offset;

		self.log(Log::Branch, format!("pc => r{register}{pc_offset:+} ({:#010X})", self.registers[0xF]));
	}

	pub fn thumb_branch(&mut self, offset: i32) {
		let (address, _) = self.registers[0xF].overflowing_add_signed(offset);

		self.registers[0xF] = address + 0x4;

		self.log(Log::Branch, format!("pc => pc{offset:+}+4 ({:#010X})", self.registers[0xF]));
	}

	pub fn thumb_branch_exchange(&mut self, register: u8) {
		let value = self.registers[register as usize];

		let t = value & 0b00000000000000000000000000000001 != 0x0;

		self.cpsr = self.cpsr & 0b11111111111111111111111111011111 | (t as u32) << 0x5;
		self.exchange(t);

		// Add extra offset to move to the new fetch
		// instruction.
		let pc_offset: u32 = match t {
			false => 0x8,
			true  => 0x4,
		};

		self.registers[0xF] = (value & 0b11111111111111111111111111111110) + pc_offset;

		self.log(Log::Branch, format!("pc => r{register}{pc_offset:+} ({:#010X})", self.registers[0xF]));
	}

	pub fn thumb_branch_link0(&mut self, offset: i32) {
		let (address, _) = self.registers[0xF].overflowing_add_signed(offset);
		self.registers[0xE] = address;

		self.log(Log::Branch, format!("lr => pc{offset:+}+4 ({:#010X})", self.registers[0xF]));
	}

	pub fn thumb_branch_link1(&mut self, offset: i32) {
		let (address, _) = self.registers[0xE].overflowing_add_signed(offset);

		self.thumb_link();

		(self.registers[0xF], _) = address.overflowing_add(0x4);

		self.log(Log::Branch, format!("pc => pc{offset:+}+4 ({:#010X})", self.registers[0xF]));
	}

}
