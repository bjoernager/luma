// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::VER;
use crate::luma::app::App;
use crate::luma::emu::Emu;

use std::fs::File;
use std::io::Read;

impl App {
	pub fn run(&mut self) {
		eprintln!("luma {VER}");

		self.prspar();

		self.ini();

		let mut emu = Emu::new();

		eprintln!("loading booatloader \"{}\"",self.btl);

		// Open bootloader:
		let mut btl = File::open(self.btl.clone()).expect("unable to open bootloader");

		// Read bootloader:
		btl.read(emu.btl()).expect("unable to read bootloader");

		eprintln!("loading image \"{}\"",self.img);

		// Open image:
		let mut img = File::open(self.img.clone()).expect("unable to open image");

		// Read image:
		img.read(emu.img()).expect("unable to read image");

		emu.run();
	}
}
