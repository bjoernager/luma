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
	pub fn thumb_load_immediate(&mut self, destination: u8, base: u8, immediate: u8) {
		// Load word from memory using immediate offset.

		let base_value = self.registers[base as usize];

		let offset = (immediate as u32) << 0x2;

		let address = base_value + offset;

		let value = self.read_word(address);
		self.registers[destination as usize] = value;

		self.log(Log::Load, format!("r{destination} => r{base}{offset:+}={address:#010X} ({value:#010X})"));
	}

	pub fn thumb_load_register(&mut self, destination: u8, base: u8, offset: u8) {
		// Load word from memory using register offset.

		let base_value   = self.registers[base   as usize];
		let offset_value = self.registers[offset as usize] as i32;

		let (address, _) = base_value.overflowing_add_signed(offset_value);

		let value = self.read_word(address);
		self.registers[destination as usize] = value;

		self.log(Log::Load, format!("r{destination} => r{base}+r{offset}={address:#010X} ({value:#010X})"));
	}

	pub fn thumb_load_pc(&mut self, destination: u8, immediate: u8) {
		// Load word from memory using offset relative to
		// the program counter.

		let offset = (immediate as u32) << 0x2;

		let base = self.registers[0xF] & 0b11111111111111111111111111111100;

		let address = base + offset;

		let value = self.read_word(address);
		self.registers[destination as usize] = value;

		self.log(Log::Load, format!("r{destination} => pc{offset:+}={address:#010X} ({value:#010X})"));
	}

	pub fn thumb_load_sp(&mut self, destination: u8, immediate: u8) {
		// Load word from memory using offset relative to
		// the stack pointer.

		let offset = (immediate as u32) << 0x2;

		let base = self.registers[0xD];

		let address = base + offset;

		let value = self.read_word(address);
		self.registers[destination as usize] = value;

		self.log(Log::Load, format!("r{destination} => sp{offset:+}={address:#010X} ({value:#010X})"));
	}
}
