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
	pub fn thumb_shift_left(&mut self, destination: u8, source: u8, immediate: u8) {
		let source_value = self.registers[source as usize];

		let (value, _) = source_value.overflowing_shl(immediate as u32);

		self.registers[destination as usize] = value;

		// TO-DO: Set condition flags.

		self.log(Log::Shift, format!("r{destination} => r{source} << {immediate} ({value:#010X})"));
	}

	pub fn thumb_shift_right(&mut self, destination: u8, source: u8, immediate: u8) {
		let source_value = self.registers[source as usize];

		let (value, _) = source_value.overflowing_shr(immediate as u32);

		self.registers[destination as usize] = value;

		// TO-DO: Set condition flags.

		self.log(Log::Shift, format!("r{destination} => r{source} << {immediate} ({value:#010X})"));
	}
}
