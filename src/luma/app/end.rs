// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::MEMSIZ;
use crate::luma::app::App;

use std::alloc::{dealloc, Layout};
use std::process::exit;

impl App {
	pub fn end(&mut self, cod: u8, msg: Option<&str>) {
		if cod != 0x0 {
			eprintln!("error: {}", msg.unwrap());
		}

		eprintln!("ending");

		unsafe { dealloc(self.mem, Layout::new::<[u32; MEMSIZ/0x20usize]>()) };

		exit(cod as i32);
	}
}
