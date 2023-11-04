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
	Affero General Public License along with Luma.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::luma::VERSION;
use crate::luma::app::App;
use crate::luma::state::State;

use std::fs::File;
use std::io::Read;

impl App {
	pub fn load(&mut self, state: &mut State) -> Result<(), String> {
		eprintln!("loading booatloader \"{}\"", self.bootloader);

		let mut bootloader = match File::open(&self.bootloader) {
			Ok(file) => file,
			_        => return Err("unable to open bootloader".to_string()),
		};

		if let Err(..) = bootloader.read(state.bootloader_buffer()) { return Err("unable to read bootloader".to_string()) };

		eprintln!("loading image \"{}\"", self.image);

		let mut image = match File::open(&self.image) {
			Ok(file) => file,
			_        => return Err("unable to open image".to_string()),
		};

		match image.read(state.image_buffer()) {
			Err(..) => return Err("unable to read image".to_string()),
			_       => {},
		};

		let title   = get_title(&state.image_buffer()[0xA0..0xAC]);
		let id      = get_id(&state.image_buffer()[0xAC..0xB0]);
		let version = state.image_buffer()[0xBC];

		eprintln!("loaded image \"{title}\" ({id}) v.{version}");

		self.canvas.window_mut().set_title(&format!("Luma {:X}.{:X} - {title}", VERSION.0, VERSION.1)).unwrap();

		return Ok(());
	}
}

fn get_title(data: &[u8]) -> String {
	let mut title = String::with_capacity(0xC);

	for raw in data {
		let character = match char::from_u32(*raw as u32) {
			Some('\u{0000}') => break,
			Some(character)  => character,
			None             => '?',
		};

		title.push(character);
	}

	return title;
}

fn get_id(data: &[u8]) -> String {
	let mut id = String::with_capacity(0xC);

	for raw in data {
		let character = match char::from_u32(*raw as u32) {
			Some('\u{0000}') => break,
			Some(character)  => character,
			None             => '?',
		};

		id.push(character);
	}

	return id;
}
