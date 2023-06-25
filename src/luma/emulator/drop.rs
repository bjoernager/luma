// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::MEMORY_SIZE;
use crate::luma::emulator::Emulator;

use std::alloc::{dealloc, Layout};
use std::mem::size_of;

impl Drop for Emulator {
	fn drop(&mut self) {
		unsafe { dealloc(self.memory, Layout::new::<[u32; MEMORY_SIZE / size_of::<u32>()]>()) };
	}
}
