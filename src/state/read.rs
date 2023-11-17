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

use crate::{Error, MEMORY_LENGTH};
use crate::cpu_mode::CpuMode;
use crate::state::State;

const MAX_BYTE_ADDRESS:     u32 = MEMORY_LENGTH;
const MAX_HALFWORD_ADDRESS: u32 = MEMORY_LENGTH - 0x1;
const MAX_WORD_ADDRESS:     u32 = MEMORY_LENGTH - 0x3;

impl State {
	#[inline(always)]
	#[must_use]
	pub fn read_register(&self, register: u8) -> u32 {
		// Limit to 0..=15.
		let index = (register & 0b00001111) as usize;

		return unsafe { **self.registers.get_unchecked(index) };
	}

	#[must_use]
	pub fn read_word(&self, address: u32) -> u32 {
		if address > MAX_WORD_ADDRESS { Error::OutOfBounds( address).trap();      return 0xFFFFFFFF; }
		if address % 0x4 != 0x0       { Error::BadAlignment(address, 0x4).trap(); return 0xFFFFFFFF; }

		unsafe {
			let pointer = (self.memory.as_ptr() as *const u8).add(address as usize) as *const u32;
			return *pointer;
		}
	}

	#[must_use]
	pub fn read_halfword(&self, address: u32) -> u16 {
		if address > MAX_HALFWORD_ADDRESS { Error::OutOfBounds( address).trap();      return 0xFFFF; }
		if address % 0x2 != 0x0           { Error::BadAlignment(address, 0x2).trap(); return 0xFFFF; }

		unsafe {
			let pointer = (self.memory.as_ptr() as *const u8).add(address as usize) as *const u16;
			return *pointer;
		}
	}

	#[must_use]
	pub fn read_byte(&self, address: u32) -> u8 {
		if address > MAX_BYTE_ADDRESS { Error::OutOfBounds(address).trap(); return 0xFF; }

		unsafe {
			let pointer = (self.memory.as_ptr() as *const u8).add(address as usize);
			return *pointer;
		}
	}

	#[inline(always)]
	#[must_use]
	pub fn read_cpsr(&self) -> u32 {
		return self.cpsr;
	}

	#[inline(always)]
	#[must_use]
	pub fn read_spsr(&self, mode: CpuMode) -> u32 {
		return unsafe { *self.spsr.get_unchecked(Self::spsr_index(mode)) };
	}
}
