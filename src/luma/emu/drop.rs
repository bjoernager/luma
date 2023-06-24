// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::MEMSIZ;
use crate::luma::emu::Emu;

use std::alloc::{dealloc, Layout};
use std::mem::size_of;

impl Drop for Emu {
	fn drop(&mut self) {
		unsafe { dealloc(self.mem, Layout::new::<[u32; MEMSIZ / size_of::<u32>()]>()) };
	}
}
