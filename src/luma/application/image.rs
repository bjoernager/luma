// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::Application;
use crate::luma::IMAGE_SIZE;

use std::slice;

impl Application {
	pub fn image<'a>(&mut self) -> &'a mut [u8] {
		return unsafe { slice::from_raw_parts_mut(self.memory.offset(0x08000000), IMAGE_SIZE) };
	}
}