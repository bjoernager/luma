// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::app::App;

impl App {
	pub fn new() -> App {
		return App {
			btl: "bootloader.bin".to_string(),
			img: "image.agb".to_string(),
		};
	}
}
