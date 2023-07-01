/*
	Copyright 2021-2023 Gabriel Jensen.

	This file is part of Luma.

	Luma is free software: you can redistribute it 
	and/or modify it under the terms of the GNU 
	Affero General Public License as published by
	the Free Software Foundation, either version 3 
	of the License, or (at your option) any later 
	version.

	Luma is distributed in the hope that it will be 
	useful, but WITHOUT ANY WARRANTY; without even 
	the implied warranty of MERCHANTABILITY or 
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU 
	Affero General Public License for more details.

	You should have received a copy of the GNU 
	Affero General Public License along with Luma. If not, 
	see <https://www.gnu.org/licenses/>. 
*/

use crate::luma::application::Application;

use std::fs::File;
use std::io::Read;

impl Application {
	pub fn load(&mut self) {
		eprintln!("loading booatloader \"{}\"", self.configuration.bootloader);

		// Open bootloader:
		let mut bootloader = File::open(self.configuration.bootloader.clone()).expect("unable to open bootloader");

		// Read bootloader:
		bootloader.read(self.device.bootloader()).expect("unable to read bootloader");

		eprintln!("loading image \"{}\"", self.configuration.image);

		// Open image:
		let mut image = File::open(self.configuration.image.clone()).expect("unable to open image");

		// Read image:
		image.read(self.device.image()).expect("unable to read image");
	}
}
