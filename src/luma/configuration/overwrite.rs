// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::configuration::Configuration;

use std::env::args;

impl Configuration {
	pub(super) fn overwrite(&mut self) {
		eprintln!("overwritting settings");

		let parameters: Vec<String> = args().collect();
		let number                  = parameters.len();

		if number >= 0x2 { self.image = parameters[0x1].clone() }

		if number >= 0x3 { self.bootloader = parameters[0x2].clone() }

		if number > 0x3 { panic!("invalid number of parameters ({number})") }
	}
}
