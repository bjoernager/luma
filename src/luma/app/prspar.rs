// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::app::App;

use std::env::args;

impl App {
	pub fn prspar(&mut self) {
		eprintln!("parsing parameters");

		let arg: Vec<String> = args().collect();
		let num              = arg.len();

		if num >= 0x2 {
			self.img = arg[0x1].clone();
		}

		if num >= 0x3 {
			self.btl = arg[0x2].clone();
		}

		if num > 0x3 {
			self.end(0x1,Some(format!("invalid number of parameters ({num})").as_str()));
		}
	}
}
