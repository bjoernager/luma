// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::CONFIGURATION_VERSION;
use crate::luma::configuration::Configuration;

use std::fs::write;

impl Configuration {
	pub(super) fn create(&mut self) {
		let configuration_path = Configuration::path();

		eprintln!("creating configuration at {configuration_path}");

		let default_configuration = format!(
			"[luma]\n\
			 version = {CONFIGURATION_VERSION}\n\
			 \n\
			 [device]\n\
			 #bootloader = \n\
			 #image = \n\
			 \n\
			 [video]\n\
			 scale = 1\n"
		);

		write(configuration_path, default_configuration).unwrap();
	}
}
