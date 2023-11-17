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

use crate::{Error, log_assignment, MEMORY_LENGTH};
use crate::cpu_mode::CpuMode;
use crate::state::{address_unused, State};

macro_rules! read_only {
	($address: expr) => {{
		match $address {
			0x00000000..=0x00003FFF => true,
			0x04000130..=0x04000131 => true, // KEYINPUT
			0x08000000..=0x0DFFFFFF => true,

			_ => false,
		}
	}};
}

impl State {
	#[inline(always)]
	pub fn write_register(&mut self, register: u8, value: u32) {
		log_assignment!(format!("r{register}"), format!("{value:#010X}"));

		let index = (register & 0b00001111) as usize;

		unsafe { **self.registers.get_unchecked_mut(index) = value };
	}

	pub fn write_word(&mut self, address: u32, value: u32) {
		log_assignment!(format!("{address:#010X}"), format!("{value:#010X}"));

		if address > MEMORY_LENGTH - 0x4 { Error::OutOfBounds( address).trap();      return; }
		if address % 0x4 != 0x0          { Error::BadAlignment(address, 0x4).trap(); return; }

		if read_only!(address) || address_unused!(address) { return };

		unsafe {
			let pointer = (self.memory.as_mut_ptr() as *mut u8).add(address as usize) as *mut u32;
			*pointer = value;
		}
	}

	pub fn write_halfword(&mut self, address: u32, value: u16) {
		log_assignment!(format!("{address:#010X}"), format!("{value:#06X}"));

		if address > MEMORY_LENGTH - 0x2 { Error::OutOfBounds( address).trap();      return; }
		if address % 0x2 != 0x0          { Error::BadAlignment(address, 0x2).trap(); return; }

		if read_only!(address) || address_unused!(address) { return };

		unsafe {
			let pointer = (self.memory.as_mut_ptr() as *mut u8).add(address as usize) as *mut u16;
			*pointer = value;
		}
	}

	pub fn write_byte(&mut self, address: u32, value: u8) {
		log_assignment!(format!("{address:#010X}"), format!("{value:#04X}"));

		if address > MEMORY_LENGTH - 0x1 { Error::OutOfBounds(address).trap(); return; }

		if read_only!(address) || address_unused!(address) { return };

		let memory = self.memory.as_mut_ptr() as *mut u8;

		match address {
			// Extend to halfwords:
			| 0x05000000..=0x050003FF
			| 0x06000000..=0x06017FFF
			| 0x07000000..=0x070003FF => {
				// Align to halfwords.
				let address = address & 0b11111111111111111111111111111110;

				// Repeat value.
				let value = value as u16 | (value as u16) << 0x8;

				unsafe {
					let pointer = memory.add(address as usize) as *mut u16;
					*pointer = value;
				}
			},

			// Bytes are allowed:
			_ => unsafe {
				let pointer = memory.add(address as usize);
				*pointer = value;
			},
		};
	}

	#[inline(always)]
	pub fn write_cpsr(&mut self, value: u32) {
		log_assignment!("cpsr", format!("{value:#034b}"));

		self.cpsr = value;
	}

	#[inline(always)]
	pub fn write_spsr(&mut self, mode: CpuMode, value: u32) {
		log_assignment!("spsr", format!("{value:#034b}"));

		unsafe { *self.spsr.get_unchecked_mut(Self::spsr_index(mode)) = value };
	}
}
