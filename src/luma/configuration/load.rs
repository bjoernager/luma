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

use crate::luma::CONFIGURATION_VERSION;
use crate::luma::configuration::Configuration;

extern crate serde;
extern crate toml;

use std::env::var;
use std::fs::read_to_string;
use serde::Deserialize;

#[derive(Deserialize)]
struct Container {
	luma:   Luma,
	device: Device,
	video:  Video,
}

#[derive(Deserialize)]
struct Luma {
	version: Option<u32>,
}

#[derive(Deserialize)]
struct Device {
	bootloader: Option<String>,
	image:      Option<String>,
}

#[derive(Deserialize)]
struct Video {
	scale: Option<u32>,
}

impl Configuration {
	pub(super) fn load(&mut self) {
		let configuration_path = Configuration::path();

		eprintln!("loading configuration \"{configuration_path}\"");

		let contents = match read_to_string(configuration_path) {
			Ok( contents) => contents,
			Err(_)        => {
				eprintln!("unable to read configuration file");
				return self.create();
			},
		};

		let configuration: Container = toml::from_str(contents.as_str()).expect("unable to parse configuration file");

		let version = configuration.luma.version.expect("missing value 'version' under 'luma'");
		if version < CONFIGURATION_VERSION { panic!("ancient version: downgrade configuration") }
		if version > CONFIGURATION_VERSION { panic!("out-of-date version: upgrade configuration") }

		let get_path = |output_path: &mut String, input_path: Option<String>| {
			match input_path {
				Some(path) => {
						*output_path = if path.chars().nth(0x0).unwrap() != '/' {
							let home_directory = match var("HOME") {
								Ok( path) => path,
								Err(_)    => { eprintln!("unable to get home directory"); "".to_string() },
							};

							home_directory + "/" + &path
						} else { path }
					},
				None => {},
			};
		};

		get_path(&mut self.bootloader, configuration.device.bootloader);
		get_path(&mut self.image,      configuration.device.image);

		if configuration.video.scale.is_some() {
			self.scale = configuration.video.scale.unwrap();
			assert!(self.scale >= 0x1);
		}
	}
}
