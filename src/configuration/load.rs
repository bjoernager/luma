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

use crate::configuration::Configuration;

extern crate toml;

use std::fs::read_to_string;
use std::str::FromStr;
use toml::{Table, Value};

impl Configuration {
	pub fn load(path: &str) -> Result<Self, String> {
		eprintln!("loading configuration at \"{path}\"");

		let configuration_text = match read_to_string(path) {
			Ok( content) => content,
			_            => return Err("unable to read file".to_string()),
		};

		let base_table = match Table::from_str(&configuration_text) {
			Ok( table) => table,
			_          => return Err("unable to parse configuration".to_string()),
		};

		let luma_table   = get_table(&base_table, "luma")?;
		let device_table = get_table(&base_table, "device")?;
		let video_table  = get_table(&base_table, "video")?;

		let version = get_integer(&luma_table, "version")?;

		if version < Self::VERSION { return Err(format!("ancient version - got {}, expected {}", version, Self::VERSION)) }
		if version > Self::VERSION { return Err(format!("out-of-date version - got {}, expected {}", version, Self::VERSION)) }

		let bootloader = get_string(&device_table, "bootloader")?;
		let image      = get_string(&device_table, "image")?;

		let scale = get_integer(&video_table, "scale")?;

		let configuration = Configuration {
			bootloader: bootloader.clone(),
			image:      image.clone(),

			scale: scale,
		};

		configuration.validate()?;
		return Ok(configuration);
	}
}

fn get_value<'a>(table: &'a Table, name: &str) -> Option<&'a Value> {
	if !table.contains_key(name) { return None };

	return Some(&table[name]);
}

fn get_table<'a>(table: &'a Table, name: &str) -> Result<&'a Table, String> {
	return match get_value(table, name) {
		Some(Value::Table(table)) => Ok(table),
		Some(_)                   => Err(format!("\"{name}\" should be a section")),
		_                         => Err("section \"{name}\" is required".to_string()),
	};
}

fn get_integer(table: &Table, name: &str) -> Result<u32, String> {
	return match get_value(table, name) {
		Some(Value::Integer(value)) => Ok(*value as u32),
		Some(_)                     => Err(format!("\"{name}\" should be an integer")),
		_                           => Err("missing integer \"{name}\"".to_string()),
	};
}

fn get_string<'a>(table: &'a Table, name: &str) -> Result<&'a String, String> {
	return match get_value(table, name) {
		Some(Value::String(string)) => Ok(string),
		Some(_)                     => Err(format!("\"{name}\" should be a string")),
		_                           => Err("missing string \"{name}\"".to_string()),
	};
}
