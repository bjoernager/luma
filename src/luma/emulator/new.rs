// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::MEMORY_SIZE;
use crate::luma::emulator::Emulator;

use std::alloc::{alloc_zeroed, Layout};
use std::mem::size_of;

impl Emulator {
	pub fn new() -> Emulator {
		let memory = unsafe { alloc_zeroed(Layout::new::<[u32; MEMORY_SIZE / size_of::<u32>()]>()) };
		if memory.is_null() { panic!("unable to allocate memory buffer") }

		eprintln!("allocated memory buffer at 0x{:0X}", memory as usize);

		return Emulator {
			memory: memory,
			registers: [
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
				0x08000008,
			],
			psr: 0b00000000000000000000000000001111,
		};
	}
}
