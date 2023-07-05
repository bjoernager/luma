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
	pub fn arm_link(&mut self) {
		// Store the address of the following instruction
		// in the link register.

		(self.registers[0xE], _) = self.registers[0xF].overflowing_sub(0x4);

		self.log(Log::Link, format!("lr => pc-4 ({:#010X})", self.registers[0xE]));
	}

	pub fn thumb_link(&mut self) {
		// Store the address of the following instruction
		// in the link register.

		(self.registers[0xE], _) = self.registers[0xF].overflowing_sub(0x2);

		self.log(Log::Link, format!("lr => pc ({:#010X})", self.registers[0xE]));
	}
}
