// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::configuration::Configuration;

impl Configuration {
	pub fn new() -> Configuration {
		let mut configuration = Configuration {
			bootloader: "bootloader.bin".to_string(),
			image:      "image.agb".to_string(),
			scale:      0x1,
		};

		configuration.load();
		configuration.overwrite();

		return configuration;
	}
}
