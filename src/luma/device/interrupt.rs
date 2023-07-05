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
	pub fn interrupt(&mut self, immediate: u32) {
		self.log(Log::Interrupt, format!("{immediate:#010X}"));

		self.spsr[0b0011] = self.cpsr;
		self.log(Log::Interrupt, format!("spsr_svc => cpsr ({:#034b})", self.spsr[0b0011]));

		// Enter svc mode.
		// Enter ARM state.
		// Disable IRQ exceptions.
		self.cpsr = self.cpsr & 0b11111111111111111111111101000000 | 0b00000000000000000000000010010011;
		self.log(Log::Interrupt, format!("cpsr => {:#034b}", self.cpsr));

		self.exchange(false);

		self.registers[0xF] = 0x00000008;
		self.log(Log::Interrupt, format!("pc => {:#010X}", self.registers[0xF]));
	}
}
