// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::MEMSIZ;
use crate::luma::app::App;

use std::alloc::{alloc_zeroed, Layout};

impl App {
	pub fn ini(&mut self) {
		eprintln!("initialising");

		self.mem = unsafe { alloc_zeroed(Layout::new::<[u32; MEMSIZ]>()) };

		eprintln!("allocated memory buffer at 0x{:0X}", self.mem as usize);
	}
}
