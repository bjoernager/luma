// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::Application;
use crate::luma::TrapKind;

impl Application {
	pub fn decode(&mut self, opcode: u32) {
		let condition = match opcode & 0b11110000000000000000000000000 {
			0b00000000000000000000000000000 => self.psr & 0b01000000000000000000000000000000 != 0x00,
			0b00010000000000000000000000000 => self.psr & 0b01000000000000000000000000000000 == 0x00,
			0b00100000000000000000000000000 => self.psr & 0b00100000000000000000000000000000 != 0x00,
			0b00110000000000000000000000000 => self.psr & 0b00100000000000000000000000000000 == 0x00,
			0b01000000000000000000000000000 => self.psr & 0b10000000000000000000000000000000 != 0x00,
			0b01010000000000000000000000000 => self.psr & 0b10000000000000000000000000000000 == 0x00,
			0b01100000000000000000000000000 => self.psr & 0b00010000000000000000000000000000 != 0x00,
			0b01110000000000000000000000000 => self.psr & 0b00010000000000000000000000000000 == 0x00,
			0b10000000000000000000000000000 => self.psr & 0b00100000000000000000000000000000 != 0x00 && self.psr & 0b01000000000000000000000000000000 == 0x00,
			0b10010000000000000000000000000 => self.psr & 0b00100000000000000000000000000000 == 0x00 && self.psr & 0b01000000000000000000000000000000 != 0x00,
			0b10100000000000000000000000000 => self.psr & 0b00010000000000000000000000000000 >> 0x1C == self.psr & 0b10000000000000000000000000000000 >> 0x1F,
			0b10110000000000000000000000000 => self.psr & 0b00010000000000000000000000000000 >> 0x1C != self.psr & 0b10000000000000000000000000000000 >> 0x1F,
			0b11000000000000000000000000000 => self.psr & 0b01000000000000000000000000000000 == 0x00 && self.psr & 0b00010000000000000000000000000000 >> 0x1C == self.psr & 0b10000000000000000000000000000000 >> 0x1F,
			0b11010000000000000000000000000 => self.psr & 0b01000000000000000000000000000000 != 0x00 || self.psr & 0b00010000000000000000000000000000 >> 0x1C != self.psr & 0b10000000000000000000000000000000 >> 0x1F,
			0b11100000000000000000000000000 => true,
			_                               => { self.trap(TrapKind::InvalidOpcode(self.registers[0xF] - 0x8, opcode)); false },
		};
		if !condition { return }

		if opcode & 0b00001110000000000000000000000000 == 0b00001010000000000000000000000000 {
			let off = opcode          & 0b00000000111111111111111111111111; // Offset from pc.
			let inv = !(opcode - 0x1) & 0b00000000111111111111111111111111; // Inverted offset.

			if opcode & 0b00000001000000000000000000000000 != 0x0 {
				self.registers[0xE] = self.registers[0xF] - 0x4;
				eprintln!("link: lr => {}", self.registers[0xE]);
			}

			self.registers[0xF] = match (off & 0b00000000100000000000000000000000) != 0x0 { // If negative...
				false => self.registers[0xF] + off * 0x4 + 0x8,
				true  => self.registers[0xF] - inv * 0x4 + 0x8,
			};

			eprintln!("branch: {off:024b} => {:08X}", self.registers[0xF] - 0x8);
			return;
		}

		self.trap(TrapKind::InvalidOpcode(self.registers[0xF] - 0x8, opcode));
	}
}
