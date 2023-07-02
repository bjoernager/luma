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

use crate::luma::device::{Device, Move};

impl Device {
	pub fn r#move(&mut self, destination: u8, kind: Move, s: bool) {
		let value = match kind {
			Move::Immediate(immediate) => immediate as u32,
			Move::Register( source)    => self.registers[source as usize],
		};

		self.registers[destination as usize] = value;

		if s { // Check the s flag.
			if destination == 0xF {
				self.cpsr = self.spsr[(self.cpsr & 0b00000000000000000000000000001111) as usize]; // We ignore the fifth bit, as this is always set.
			} else {
				// TO-DO
			}
		}

		self.log("move", match kind {
			Move::Immediate(..)     => format!("r{destination} => {value:#04X}"),
			Move::Register( source) => format!("r{destination} => r{source} ({value:#010X})"),
		});
	}
}