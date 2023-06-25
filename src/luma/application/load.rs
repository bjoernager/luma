// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::Application;

use std::fs::File;
use std::io::Read;

impl Application {
	pub fn load(&mut self) {
		eprintln!("loading booatloader \"{}\"", self.bootloader);

		// Open bootloader:
		let mut bootloader = File::open(self.bootloader.clone()).expect("unable to open bootloader");

		// Read bootloader:
		bootloader.read(self.bootloader()).expect("unable to read bootloader");

		eprintln!("loading image \"{}\"", self.image);

		// Open image:
		let mut image = File::open(self.image.clone()).expect("unable to open image");

		// Read image:
		image.read(self.image()).expect("unable to read image");
	}
}
