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

use crate::luma::device::{Device, Trap};

impl Device {
	pub fn decode(&mut self) {
		let opcode = self.read_word(self.registers[0xF] - 0x8);

		let condition = match opcode & 0b11110000000000000000000000000000 {
			0b00000000000000000000000000000000 => self.cpsr & 0b01000000000000000000000000000000 != 0x00,
			0b00010000000000000000000000000000 => self.cpsr & 0b01000000000000000000000000000000 == 0x00,
			0b00100000000000000000000000000000 => self.cpsr & 0b00100000000000000000000000000000 != 0x00,
			0b00110000000000000000000000000000 => self.cpsr & 0b00100000000000000000000000000000 == 0x00,
			0b01000000000000000000000000000000 => self.cpsr & 0b10000000000000000000000000000000 != 0x00,
			0b01010000000000000000000000000000 => self.cpsr & 0b10000000000000000000000000000000 == 0x00,
			0b01100000000000000000000000000000 => self.cpsr & 0b00010000000000000000000000000000 != 0x00,
			0b01110000000000000000000000000000 => self.cpsr & 0b00010000000000000000000000000000 == 0x00,
			0b10000000000000000000000000000000 => self.cpsr & 0b00100000000000000000000000000000 != 0x00 && self.cpsr & 0b01000000000000000000000000000000 == 0x00,
			0b10010000000000000000000000000000 => self.cpsr & 0b00100000000000000000000000000000 == 0x00 && self.cpsr & 0b01000000000000000000000000000000 != 0x00,
			0b10100000000000000000000000000000 => self.cpsr & 0b00010000000000000000000000000000 >> 0x1C == self.cpsr & 0b10000000000000000000000000000000 >> 0x1F,
			0b10110000000000000000000000000000 => self.cpsr & 0b00010000000000000000000000000000 >> 0x1C != self.cpsr & 0b10000000000000000000000000000000 >> 0x1F,
			0b11000000000000000000000000000000 => self.cpsr & 0b01000000000000000000000000000000 == 0x00 && self.cpsr & 0b00010000000000000000000000000000 >> 0x1C == self.cpsr & 0b10000000000000000000000000000000 >> 0x1F,
			0b11010000000000000000000000000000 => self.cpsr & 0b01000000000000000000000000000000 != 0x00 || self.cpsr & 0b00010000000000000000000000000000 >> 0x1C != self.cpsr & 0b10000000000000000000000000000000 >> 0x1F,
			0b11100000000000000000000000000000 => true,
			_                                  => return self.trap(Trap::InvalidOpcode(self.registers[0xF] - 0x8, opcode)),
		};
		if !condition { return self.r#continue() }

		// load/store
		if opcode & 0b00001111001000000000000000000000 == 0b00000101000000000000000000000000 {
			let register = ((opcode & 0b00000000000000001111000000000000) >> 0xC) as u8;

			let base = ((opcode & 0b00000000000011110000000000000000) >> 0x10) as u8; 

			let immediate = (opcode & 0b00000000000000000000111111111111) as u16;

			let u = 0b00000000100000000000000000000000 != 0x0;
			let b = 0b00000000010000000000000000000000 != 0x0;
			let l = 0b00000000000100000000000000000000 != 0x0;

			self.store(register, base, immediate, u, b, l);
			return self.r#continue();
		}

		// move
		if opcode & 0b00001101111111100000111111110000 == 0b00000001101000000000000000000000 {
			let destination = ((opcode & 0b00000000000000001111000000000000) >> 0xC) as u8;
			let source      =  (opcode & 0b00000000000000000000000000001111)         as u8;

			let value                            = self.registers[source as usize];
			self.registers[destination as usize] = value;

			let s = opcode & 0b00000000000100000000000000000000 != 0x0;
			
			self.r#move(destination, source, s);
			return self.r#continue();
		}

		// branch
		if opcode & 0b00001110000000000000000000000000 == 0b00001010000000000000000000000000 {
			let link = opcode & 0b00000001000000000000000000000000 != 0x0;
			
			let offset = {
				let mut offset = opcode & 0b00000000111111111111111111111111;

				if offset & 0b00000000100000000000000000000000 != 0x0 { offset |= 0b00111111000000000000000000000000 } // Sign-extend.

				offset <<= 0x2;

				offset as i32
			};

			return self.branch(offset, link);
		}

		self.trap(Trap::InvalidOpcode(self.registers[0xF] - 0x8, opcode));

		self.r#continue();
	}
}
