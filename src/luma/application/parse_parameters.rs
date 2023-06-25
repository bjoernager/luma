// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::Application;

use std::env::args;

impl Application {
	pub fn parse_parameters(&mut self) {
		eprintln!("parsing parameters");

		let parameters: Vec<String> = args().collect();
		let number                  = parameters.len();

		if number >= 0x2 {
			self.image = parameters[0x1].clone();
		}

		if number >= 0x3 {
			self.bootloader = parameters[0x2].clone();
		}

		if number > 0x3 {
			self.end(0x1, Some(format!("invalid number of parameters ({number})").as_str()));
		}
	}
}
