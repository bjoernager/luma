// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::app::App;

use std::process::exit;

impl App {
	pub fn end(&mut self, cod: u8, msg: Option<&str>) {
		if cod != 0x0 {
			eprintln!("error: {}", msg.unwrap());
		}

		eprintln!("ending");

		exit(cod as i32);
	}
}
