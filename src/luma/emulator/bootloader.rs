// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::BOOTLOADER_SIZE;
use crate::luma::emulator::Emulator;

use std::slice;

impl Emulator {
	pub fn bootloader<'a>(&mut self) -> &'a mut [u8] {
		return unsafe { slice::from_raw_parts_mut(self.memory.offset(0x00000000), BOOTLOADER_SIZE) };
	}
}
