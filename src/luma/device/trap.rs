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

use crate::luma::MEMORY_SIZE;
use crate::luma::device::{Device, Trap};

impl Device {
	pub fn trap(&mut self, kind: Trap) {
		let message = match kind {
			Trap::BadAlignment( address, alignment) => format!("bad alignment of address {address:#010X} (should be {alignment}-byte aligned)"),
			Trap::InvalidOpcode(address, opcode)    => format!("invalid opcode {opcode:#034b} at {address:#010X}"),
			Trap::OutOfBounds(  address)            => format!("out-of-bounds address {address:#010X} (limit is {MEMORY_SIZE:#010X})"),
		};

		eprintln!("{message}");
		eprintln!("  r0:       {:#010X}", self.registers[0x0]);
		eprintln!("  r1:       {:#010X}", self.registers[0x1]);
		eprintln!("  r2:       {:#010X}", self.registers[0x2]);
		eprintln!("  r3:       {:#010X}", self.registers[0x3]);
		eprintln!("  r4:       {:#010X}", self.registers[0x4]);
		eprintln!("  r5:       {:#010X}", self.registers[0x5]);
		eprintln!("  r6:       {:#010X}", self.registers[0x6]);
		eprintln!("  r7:       {:#010X}", self.registers[0x7]);
		eprintln!("  r8:       {:#010X}", self.registers[0x8]);
		eprintln!("  r9:       {:#010X}", self.registers[0x9]);
		eprintln!("  r10:      {:#010X}", self.registers[0xA]);
		eprintln!("  r11:      {:#010X}", self.registers[0xB]);
		eprintln!("  r12:      {:#010X}", self.registers[0xC]);
		eprintln!("  r13:      {:#010X}", self.registers[0xD]);
		eprintln!("  r14:      {:#010X}", self.registers[0xE]);
		eprintln!("  r15:      {:#010X}", self.registers[0xF]);
		eprintln!("  cpsr:     {:#034b}", self.cpsr);
		eprintln!("  spsr_fiq: {:#034b}", self.spsr[0x1]);
		eprintln!("  spsr_irq: {:#034b}", self.spsr[0x2]);
		eprintln!("  spsr_svc: {:#034b}", self.spsr[0x3]);
		eprintln!("  spsr_abt: {:#034b}", self.spsr[0x7]);
		eprintln!("  spsr_und: {:#034b}", self.spsr[0xB]);
	}
}
