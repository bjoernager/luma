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

use crate::{BOOTLOADER_LENGTH, IMAGE_LENGTH, PALETTE_LENGTH, VIDEO_LENGTH};
use crate::cpu_mode::CpuMode;

use std::slice::{from_raw_parts, from_raw_parts_mut};

pub mod bank;
pub mod new;
pub mod read;
pub mod shifter_value;
pub mod write;

pub struct State {
	registers: [*mut u32; 0x10],
	banks:     Box<[[u32; 0x10]; 0x6]>,

	cpsr: u32,
	spsr: [u32; 0x6],

	memory: Vec::<u32>,
}

macro_rules! address_unused {
	($address: expr) => {{
		match $address {
			0x02040000..=0x02FFFFFF => true,
			0x03008000..=0x03FFFFFF => true,
			0x04000400..=0x04FFFFFF => true,
			0x05000400..=0x05FFFFFF => true,
			0x06018000..=0x06FFFFFF => true,
			0x07000400..=0x07FFFFFF => true,

			_ => false,
		}
	}};
}
pub(crate) use address_unused;

impl State {
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

	#[must_use]
	fn spsr_index(mode: CpuMode) -> usize {
		use CpuMode::*;

		return match mode {
			User                 => 0x0,
			System               => 0x0,
			FastInterruptRequest => 0x1,
			InterruptRequest     => 0x2,
			Supervisor           => 0x3,
			Abort                => 0x4,
			Undefined            => 0x5,
		};
	}
}

unsafe impl Send for State { }
