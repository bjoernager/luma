// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::CONFIGURATION_VERSION;
use crate::luma::configuration::Configuration;

use configparser::ini::Ini;
use std::env::var;

impl Configuration {
	pub(super) fn load(&mut self) {
		let configuration_path = Configuration::path();

		eprintln!("loading configuration \"{configuration_path}\"");

		let mut configuration = Ini::new();

		match configuration.load(configuration_path) {
			Ok( _) => {},
			Err(_) => {
				eprintln!("unable to read configuration");

				return self.create();
			},
		}

		let get_path = |configuration: &mut Ini, section: &str, entry: &str| -> Option<String> {
			match configuration.get(section, entry) {
				Some(path) => {
					if path.chars().nth(0x0).unwrap() != '/' {
						let home_directory = match var("HOME") {
							Ok( path) => path,
							Err(_)    => { eprintln!("unable to get home directory"); "".to_string() },
						};

						return Some(home_directory + "/" + &path)
					}

					return Some(path);
				},
				None => None,
			}
		};

		let get_unsigned = |configuration: &mut Ini, section: &str, entry: &str| -> Option<u64> { 
			match configuration.getuint(section, entry) {
				Ok( optional) => optional,
				Err(_)        => panic!("invalid format of '{entry}' in configuration under '{section}"),
			}
		};

		let version = get_unsigned(&mut configuration, "luma", "version").expect("'version' field not defined in configuration under 'luma'") as u32;

		if version < CONFIGURATION_VERSION { panic!("out-of-date configuration - please upgrade") };
		if version > CONFIGURATION_VERSION { panic!("future configuration - please downgrade") };

		match get_path(&mut configuration, "device", "bootloader") {
			Some(path) => {
				self.bootloader = path;
			},
			None => {},
		}

		match get_path(&mut configuration, "device", "image") {
			Some(path) => {
				self.image = path;
			},
			None => {},
		}

		match get_unsigned(&mut configuration, "video", "scale") {
			Some(value) => {
				assert!(value >= 0x1);
				assert!(value <= 0xFFFFFFFF);

				self.scale = value as u32;
			},
			None => {},
		}
	}
}
