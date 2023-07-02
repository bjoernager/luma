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
use crate::luma::device::Device;

use std::alloc::{alloc_zeroed, Layout};
use std::mem::size_of;

impl Device {
	pub fn new() -> Device {
		eprintln!("creating new device");

		let memory = unsafe { alloc_zeroed(Layout::new::<[u32; MEMORY_SIZE / size_of::<u32>()]>()) };
		if memory.is_null() { panic!("unable to allocate memory buffer") }

		eprintln!("allocated memory buffer at {:#0X}", memory as usize);

		let start = 0x08000008;
		eprintln!("starting emulation at {start:#08X}");

		return Device {
			decode:        Device::decode_arm,
			memory:        memory,
			registers:     [
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				start,
			],
			cpsr:          0b00000000000000000000000000001111,
			spsr:          [0b00000000000000000000000000000000; 0x10],
		};
	}
}
