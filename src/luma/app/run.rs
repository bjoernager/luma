// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::{BTLSIZ, IMGSIZ, VER};
use crate::luma::app::App;

use std::fs::File;
use std::io::Read;
use std::slice;

impl App {
	pub fn run(&mut self) {
		eprintln!("luma {VER}");

		self.prspar();

		self.ini();

		{
			eprintln!("loading booatloader \"{}\"",self.btl);

			// Open bootloader:
			let mut btl = File::open(self.btl.clone()).expect("unable to open bootloader");

			// Read bootloader:
			let slc = unsafe { slice::from_raw_parts_mut(self.mem.offset(0x00000000), BTLSIZ) };
			btl.read(slc).expect("unable to read bootloader");
		}

		{
			eprintln!("loading image \"{}\"",self.img);

			// Open image:
			let mut img = File::open(self.img.clone()).expect("unable to open image");

			// Read image:
			let slc = unsafe { slice::from_raw_parts_mut(self.mem.offset(0x08000000), IMGSIZ) };
			img.read(slc).expect("unable to read image");
		}

		self.emu();

		self.end(0x0,None);
	}
}
