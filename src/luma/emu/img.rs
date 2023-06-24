// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::IMGSIZ;
use crate::luma::emu::Emu;

use std::slice;

impl Emu {
	pub fn img<'a>(&mut self) -> &'a mut [u8] {
		return unsafe { slice::from_raw_parts_mut(self.mem.offset(0x08000000), IMGSIZ) };
	}
}
