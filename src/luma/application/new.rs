// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::Application;

impl Application {
	pub fn new() -> Application {
		return Application {
			bootloader: "bootloader.bin".to_string(),
			image:      "image.agb".to_string(),
		};
	}
}
