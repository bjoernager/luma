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

use crate::luma::{Error, log};
use crate::luma::{BOOTLOADER_LENGTH, IMAGE_LENGTH, MEMORY_LENGTH, PALETTE_LENGTH, VIDEO_LENGTH};

use std::slice::{from_raw_parts, from_raw_parts_mut};

pub mod new;

pub struct State {
	registers: [u32; 0x10],
	cpsr:      u32,

	memory: Vec::<u32>,
}

impl State {
	#[inline(always)]
	#[must_use]
	pub fn read_register(&self, register: u8) -> u32 {
		// Limit to 0..=15.
		let index = (register & 0b00001111) as usize;

		return unsafe { *self.registers.get_unchecked(index) };
	}

	#[inline(always)]
	pub fn write_register(&mut self, register: u8, value: u32) {
		log(&format!("* r{register} = {value:#010X}"));

		let index = (register & 0b00001111) as usize;

		unsafe { *self.registers.get_unchecked_mut(index) = value };
	}

	#[must_use]
	pub fn read_word(&self, address: u32) -> u32 {
		if address > MEMORY_LENGTH - 0x4 { Error::OutOfBounds( address).trap();      return 0x00000000; }
		if address % 0x4 != 0x0          { Error::BadAlignment(address, 0x4).trap(); return 0x00000000; }

		unsafe {
			let pointer = (self.memory.as_ptr() as *const u8).add(address as usize) as *const u32;
			return *pointer;
		}
	}

	pub fn write_word(&mut self, address: u32, value: u32) {
		log(&format!("* {address:#010X} = {value:#010X}"));

		if address > MEMORY_LENGTH - 0x4 { Error::OutOfBounds( address).trap();      return; }
		if address % 0x4 != 0x0          { Error::BadAlignment(address, 0x4).trap(); return; }

		unsafe {
			let pointer = (self.memory.as_mut_ptr() as *mut u8).add(address as usize) as *mut u32;
			*pointer = value;
		}
	}

	#[must_use]
	pub fn read_halfword(&self, address: u32) -> u16 {
		if address > MEMORY_LENGTH - 0x2 { Error::OutOfBounds( address).trap();      return 0x0000; }
		if address % 0x2 != 0x0          { Error::BadAlignment(address, 0x2).trap(); return 0x0000; }

		unsafe {
			let pointer = (self.memory.as_ptr() as *const u8).add(address as usize) as *const u16;
			return *pointer;
		}
	}

	pub fn write_halfword(&mut self, address: u32, value: u16) {
		log(&format!("* {address:#010X} = {value:#010X}"));

		if address > MEMORY_LENGTH - 0x2 { Error::OutOfBounds( address).trap();      return; }
		if address % 0x2 != 0x0          { Error::BadAlignment(address, 0x2).trap(); return; }

		unsafe {
			let pointer = (self.memory.as_mut_ptr() as *mut u8).add(address as usize) as *mut u16;
			*pointer = value;
		}
	}

	#[must_use]
	pub fn read_byte(&self, address: u32) -> u8 {
		if address > MEMORY_LENGTH - 0x1 { Error::OutOfBounds(address).trap(); return 0x00; }

		unsafe {
			let pointer = (self.memory.as_ptr() as *const u8).add(address as usize);
			return *pointer;
		}
	}

	pub fn write_byte(&mut self, address: u32, value: u8) {
		log(&format!("* {address:#010X} = {value:#010X}"));

		if address > MEMORY_LENGTH - 0x1 { Error::OutOfBounds(address).trap(); return; }

		unsafe {
			let pointer = (self.memory.as_mut_ptr() as *mut u8).add(address as usize);
			*pointer = value;
		}
	}

	#[inline(always)]
	#[must_use]
	pub fn read_cpsr(&self) -> u32 {
		return self.cpsr;
	}

	#[inline(always)]
	pub fn write_cpsr(&mut self, value: u32) {
		log(&format!("* cpsr = {value:#034b}"));

		self.cpsr = value;
	}

	#[must_use]
	pub fn video8<'a>(&'a self) -> &'a [u8] {
		let slice = unsafe {
			let pointer = (self.memory.as_ptr() as *const u8).add(0x06000000);
			let slice = from_raw_parts(pointer, VIDEO_LENGTH as usize);

			slice
		};

		return slice;
	}

	#[must_use]
	pub fn palette<'a>(&'a self) -> &'a [u16] {
		let slice = unsafe {
			let pointer = (self.memory.as_ptr() as *const u8).add(0x05000000) as *const u16;
			let slice = from_raw_parts(pointer, PALETTE_LENGTH as usize);

			slice
		};

		return slice;
	}

	#[must_use]
	pub fn bootloader_buffer<'a>(&'a mut self) -> &'a mut [u8] {
		let slice = unsafe {
			let pointer = (self.memory.as_mut_ptr() as *mut u8).add(0x00000000);
			let slice = from_raw_parts_mut(pointer, BOOTLOADER_LENGTH as usize);

			slice
		};

		return slice;
	}

	#[must_use]
	pub fn image_buffer<'a>(&'a mut self) -> &'a mut [u8] {
		let slice = unsafe {
			let pointer = (self.memory.as_mut_ptr() as *mut u8).add(0x08000000);
			let slice = from_raw_parts_mut(pointer, IMAGE_LENGTH as usize);

			slice
		};

		return slice;
	}
}
