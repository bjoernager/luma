// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::app::App;

use std::ptr::null;

impl App {
	pub fn new() -> App {
		let app = App {
			btl: "bootloader.bin".to_string(),
			img: "image.agb".to_string(),
			mem: null::<u8>() as *mut u8,
		};

		return app;
	}
}
