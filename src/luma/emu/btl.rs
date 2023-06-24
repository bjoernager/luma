// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::BTLSIZ;
use crate::luma::emu::Emu;

use std::slice;

impl Emu {
	pub fn btl<'a>(&mut self) -> &'a mut [u8] {
		return unsafe { slice::from_raw_parts_mut(self.mem.offset(0x00000000), BTLSIZ) };
	}
}
