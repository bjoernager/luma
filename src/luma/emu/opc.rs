// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::emu::Emu;

use std::hint::unreachable_unchecked;

fn cndexe(cpsr: u32, cnd: u8) -> Option<bool> {
	return match cnd {
		0b0000 => Some((cpsr & 0b01000000000000000000000000000000) != 0x00),
		0b0001 => Some((cpsr & 0b01000000000000000000000000000000) == 0x00),
		0b0010 => Some((cpsr & 0b00100000000000000000000000000000) != 0x00),
		0b0011 => Some((cpsr & 0b00100000000000000000000000000000) == 0x00),
		0b0100 => Some((cpsr & 0b10000000000000000000000000000000) != 0x00),
		0b0101 => Some((cpsr & 0b10000000000000000000000000000000) == 0x00),
		0b0110 => Some((cpsr & 0b00010000000000000000000000000000) != 0x00),
		0b0111 => Some((cpsr & 0b00010000000000000000000000000000) == 0x00),
		0b1000 => Some((cpsr & 0b00100000000000000000000000000000) != 0x00 && (cpsr & 0b01000000000000000000000000000000) == 0x00),
		0b1001 => Some((cpsr & 0b00100000000000000000000000000000) == 0x00 && (cpsr & 0b01000000000000000000000000000000) != 0x00),
		0b1010 => Some((cpsr & 0b00010000000000000000000000000000) >> 0x1C == (cpsr & 0b10000000000000000000000000000000) >> 0x1F),
		0b1011 => Some((cpsr & 0b00010000000000000000000000000000) >> 0x1C != (cpsr & 0b10000000000000000000000000000000) >> 0x1F),
		0b1100 => Some((cpsr & 0b01000000000000000000000000000000) == 0x00 && (cpsr & 0b00010000000000000000000000000000) >> 0x1C == (cpsr & 0b10000000000000000000000000000000) >> 0x1F),
		0b1101 => Some((cpsr & 0b01000000000000000000000000000000) != 0x00 || (cpsr & 0b00010000000000000000000000000000) >> 0x1C != (cpsr & 0b10000000000000000000000000000000) >> 0x1F),
		0b1110 => Some(true),
		_      => None,
	}
}

impl Emu {
	pub fn opc(&mut self, opc: u32) {
		// Currently, we only support the bal.w instruction.
		if opc & 0b00001111000000000000000000000000 == 0b00001010000000000000000000000000 {
			let off = opc          & 0b00000000111111111111111111111111; // Offset from pc.
			let inv = !(opc - 0x1) & 0b00000000111111111111111111111111; // Inverted offset.

			let cnd = ((opc & 0b11110000000000000000000000000) >> 0x19) as u8;
			match cndexe(self.reg.cpsr, cnd) {
				None        => self.trpcnd(cnd),
				Some(false) => return,
				Some(true)  => {},
			}

			self.reg.pc = match (off & 0b00000000100000000000000000000000) != 0x0 { // If negative...
				false => self.reg.pc + off * 0x4 + 0x8,
				true  => self.reg.pc - inv * 0x4 + 0x8,
			};
			eprintln!("branch {off:X} => {:X}", self.reg.pc);

			return;
		}
		
		self.trpopc(self.reg.pc - 0x8, opc);
		unsafe { unreachable_unchecked() };
	}
}
