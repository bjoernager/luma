// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::VERSION;
use crate::luma::application::Application;
use crate::luma::emulator::Emulator;

use std::fs::File;
use std::io::Read;

impl Application {
	pub fn run(&mut self) {
		eprintln!("luma {VERSION}");

		self.parse_parameters();

		self.initialise();

		let mut emulator = Emulator::new();

		eprintln!("loading booatloader \"{}\"",self.bootloader);

		// Open bootloader:
		let mut bootloader = File::open(self.bootloader.clone()).expect("unable to open bootloader");

		// Read bootloader:
		bootloader.read(emulator.bootloader()).expect("unable to read bootloader");

		eprintln!("loading image \"{}\"",self.image);

		// Open image:
		let mut image = File::open(self.image.clone()).expect("unable to open image");

		// Read image:
		image.read(emulator.image()).expect("unable to read image");

		emulator.run();
	}
}
