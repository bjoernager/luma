// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::Application;
use crate::luma::MEMORY_SIZE;

use std::alloc::{dealloc, Layout};
use std::mem::size_of;

impl Drop for Application {
	fn drop(&mut self) {
		eprintln!("ending");

		unsafe { dealloc(self.memory, Layout::new::<[u32; MEMORY_SIZE / size_of::<u32>()]>()) };
	}
}
